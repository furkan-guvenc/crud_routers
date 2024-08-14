use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::routing::get;
use diesel_postgres::models::{NewPost, Post, PostForm};
use diesel_postgres::schema::posts::dsl::posts;
use diesel_postgres::schema::posts::published;


#[axum::debug_handler]
async fn list_posts_route(
    State(state): State<Arc<Mutex<AppState>>>
) -> Json<Vec<Post>> {

    let mut state = state.lock().unwrap();

    posts
        .filter(published.eq(false))
        .select(Post::as_select())
        .load(&mut state.connection)
        .expect("Error loading posts")
        .into()
}

#[axum::debug_handler]
async fn create_posts_route(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(new_post): Json<NewPost>
) -> Json<Post> {
    use diesel_postgres::schema::posts;

    let mut state = state.lock().unwrap();

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(&mut state.connection)
        .expect("Error saving new post")
        .into()
}

#[axum::debug_handler]
async fn delete_all_posts_route(
    State(state): State<Arc<Mutex<AppState>>>
) -> Json<usize> {
    let mut state = state.lock().unwrap();

    diesel::delete(posts)
        .execute(&mut state.connection)
        .expect("Error deleting posts")
        .into()
}


#[axum::debug_handler]
async fn get_post_route(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>
) -> Json<Option<Post>> {
    use diesel_postgres::schema::posts;
    let mut state = state.lock().unwrap();

    posts
        .filter(posts::id.eq(id))
        .limit(1)
        .select(Post::as_select())
        .get_result(&mut state.connection)
        .optional()
        .unwrap()
        .into()

}

#[axum::debug_handler]
async fn update_post_route(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>,
    Json(post): Json<PostForm>,
) -> Json<Post> {
    use diesel_postgres::schema::posts;

    let mut state = state.lock().unwrap();

    diesel::update(posts::table)
        .filter(posts::id.eq(id))
        .set(post)
        .returning(Post::as_returning())
        .get_result(&mut state.connection)
        .expect("Updating the post")
        .into()
}


#[axum::debug_handler]
async fn delete_post_route(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>
) {
    use diesel_postgres::schema::posts;

    let mut state = state.lock().unwrap();
    diesel::delete(posts.filter(posts::id.eq(id)))
        .execute(&mut state.connection)
        .expect("Error deleting posts");
}



pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

struct AppState {
    connection: PgConnection
}

#[tokio::main]
async fn main() {
    let app = get_app().await;

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 8008));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_app() -> Router {
    let connection = establish_connection();

    let shared_state = Arc::new(Mutex::new(AppState { connection }));

    Router::new()
        .route("/", get(list_posts_route).post(create_posts_route).delete(delete_all_posts_route))
        .route("/:id", get(get_post_route).put(update_post_route).delete(delete_post_route))
        .with_state(shared_state)
}

#[cfg(test)]
mod tests {
    use crate::get_app;

    use serde_json::{json, Value};
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt; // for `collect`
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

    fn delete_all_request () -> Request<Body> {
        Request::delete("/")
            .body(Body::empty())
            .unwrap()
    }

    fn get_post_request () -> Request<Body> {
        Request::post("/")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                serde_json::to_vec(&json!({"title": "Post", "body": "Body", "published": false})).unwrap(),
            ))
            .unwrap()
    }


    #[tokio::test]
    async fn e2e(){
        let app = get_app().await;

        // no posts in the beginning
        let response = app.clone()
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"[]");

        // insert a post
        let response = app.clone().oneshot(get_post_request())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(*body.get("title").unwrap(), json!("Post"));
        assert_eq!(*body.get("body").unwrap(), json!("Body"));
        assert_eq!(*body.get("published").unwrap(), json!(false));

        // insert 2 more and get all
        let _ = app.clone().oneshot(get_post_request())
            .await
            .unwrap();

        let _ = app.clone().oneshot(get_post_request())
            .await
            .unwrap();

        // get all 3 of them
        let response = app.clone()
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let mut body: Value = serde_json::from_slice(&body).unwrap();
        let posts = body.as_array_mut().unwrap();
        assert_eq!(posts.len(), 3);
        for body in posts.iter() {
            assert_eq!(*body.get("title").unwrap(), json!("Post"));
            assert_eq!(*body.get("body").unwrap(), json!("Body"));
            assert_eq!(*body.get("published").unwrap(), json!(false));
        }

        // update the first one
        let first_post_id = posts[0].as_object_mut().unwrap().remove("id").unwrap().as_i64().unwrap();
        *posts[0].get_mut("body").unwrap() = json!("First Post Body");

        let response = app.clone()
            .oneshot(
                Request::put(format!("/{}", first_post_id))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        posts[0].to_string(),
                    ))
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // get the updated one
        let response = app.clone()
            .oneshot(Request::get(format!("/{}", first_post_id)).body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(*body.get("id").unwrap(), json!(first_post_id));
        assert_eq!(*body.get("title").unwrap(), json!("Post"));
        assert_eq!(*body.get("body").unwrap(), json!("First Post Body"));
        assert_eq!(*body.get("published").unwrap(), json!(false));

        // delete first one
        let response = app.clone()
            .oneshot(
                Request::delete(format!("/{}", first_post_id)).body(Body::empty()).unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // try to get the deleted one
        let response = app.clone()
            .oneshot(Request::get(format!("/{}", first_post_id)).body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"null");

        // get 2 of them
        let response = app.clone()
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let mut body: Value = serde_json::from_slice(&body).unwrap();
        let posts = body.as_array_mut().unwrap();
        assert_eq!(posts.len(), 2);
        for body in posts.iter() {
            assert_eq!(*body.get("title").unwrap(), json!("Post"));
            assert_eq!(*body.get("body").unwrap(), json!("Body"));
            assert_eq!(*body.get("published").unwrap(), json!(false));
        }

        // delete all
        let response = app.clone().oneshot(delete_all_request())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // all posts should be deleted
        let response = app
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"[]");
    }

}

