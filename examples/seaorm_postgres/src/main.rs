use dotenvy::dotenv;
use std::{env, io};
use std::net::TcpListener;
use actix_web::{App, HttpServer};
use actix_web::dev::Server;
use actix_web::web::Data;
use sea_orm::*;
use tokio::sync::Mutex;
use axum_crudrouter::{ActixServer, CrudRouterBuilder, SeaOrmRepository};
use seaorm_postgres::{post as post};


async fn establish_connection() -> DatabaseConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(&database_url)
        .await
        .expect("Error connecting to database")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Could not bind TCP listener");
    run(listener).await?.await
}

pub async fn run(listener: TcpListener) -> std::io::Result<Server> {
    let connection = establish_connection().await;

    let shared_state = Data::new(Mutex::new( SeaOrmRepository::new(connection)));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .service(
                CrudRouterBuilder::new::<ActixServer>()
                    .repository::<SeaOrmRepository>()
                    .schema::<post::Model, i32>()
                    .create_schema::<post::NewPost>()
                    .update_schema::<post::PostForm>()
                    .build_router()
            )
    })
        .listen(listener)?
        .run();

    Ok(server)
}


#[cfg(test)]
mod tests {
    use std::net::TcpListener;
    use crate::run;

    use serde_json::{json, Value};
    use serde::Serialize;

    struct TestApp{
        pub address: String,
        pub api_client: reqwest::Client
    }

    impl TestApp {
        pub fn new(address: String) -> Self{
            Self{
                address,
                api_client: reqwest::Client::new()
            }
        }
        async fn list_all(&self) -> reqwest::Response {
            self.api_client.get(&self.address)
                .send()
                .await
                .expect("Failed to execute request.")
        }
        async fn get(&self, id: i64) -> reqwest::Response {
            self.api_client.get(&format!("{}/{}", &self.address, id))
                .send()
                .await
                .expect("Failed to execute request.")
        }
        async fn create(&self, body: impl Serialize) -> reqwest::Response {
            self.api_client.post(&format!("{}", &self.address))
                .header("Content-Type", mime::APPLICATION_JSON.as_ref())
                .body(reqwest::Body::from(serde_json::to_vec(&body).unwrap()))
                .send()
                .await
                .expect("Failed to execute request.")
        }
        async fn update(&self, id: i64, body: impl Serialize) -> reqwest::Response {
            self.api_client.put(&format!("{}/{}", &self.address, id))
                .body(reqwest::Body::from(serde_json::to_vec(&body).unwrap()))
                .header("Content-Type", mime::APPLICATION_JSON.as_ref())
                .send()
                .await
                .expect("Failed to execute request.")
        }
        async fn delete(&self, id: i64) -> reqwest::Response {
            self.api_client.delete(&format!("{}/{}", &self.address, id))
                .send()
                .await
                .expect("Failed to execute request.")
        }
        async fn delete_all(&self) -> reqwest::Response {
            self.api_client.delete(&self.address)
                .send()
                .await
                .expect("Failed to execute request.")
        }
    }

    async fn spawn_app() -> TestApp{
        let listener = TcpListener::bind("127.0.0.1:0")
            .expect("Could not bind TCP listener");
        let port = listener.local_addr().unwrap().port();
        let server = run(listener).await.expect("Failed to bind address");
        let _ = tokio::spawn(server);

        TestApp::new(format!("http://127.0.0.1:{}", port))
    }

    #[tokio::test]
    async fn e2e(){
        let app = spawn_app().await;

        // no posts in the beginning
        let response = app.list_all().await;

        assert!(response.status().is_success());
        assert_eq!("[]", response.text().await.unwrap());

        // insert a post
        let response = app.create(
            &json!({"title": "Post", "body": "Body", "published": false})
        ).await;

        assert!(response.status().is_success());

        let body = response.bytes().await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(*body.get("title").unwrap(), json!("Post"));
        assert_eq!(*body.get("body").unwrap(), json!("Body"));
        assert_eq!(*body.get("published").unwrap(), json!(false));

        // insert 2 more and get all
        let _ = app.create(
            &json!({"title": "Post", "body": "Body", "published": false})
        ).await;

        let _ = app.create(
            &json!({"title": "Post", "body": "Body", "published": false})
        ).await;

        // get all 3 of them
        let response = app.list_all().await;

        assert!(response.status().is_success());

        let body = response.bytes().await.unwrap();
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

        let response = app.update(first_post_id, &posts[0]).await;

        assert!(response.status().is_success());

        // get the updated one
        let response = app.get(first_post_id).await;
        assert!(response.status().is_success());
        let body = response.bytes().await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(*body.get("id").unwrap(), json!(first_post_id));
        assert_eq!(*body.get("title").unwrap(), json!("Post"));
        assert_eq!(*body.get("body").unwrap(), json!("First Post Body"));
        assert_eq!(*body.get("published").unwrap(), json!(false));

        // delete first one
        let response = app.delete(first_post_id).await;

        assert!(response.status().is_success());

        // try to get the deleted one
        let response = app.get(first_post_id).await;

        assert!(response.status().is_success());
        let body = response.bytes().await.unwrap();
        assert_eq!(&body[..], b"null");

        // get 2 of them
        let response = app.list_all().await;

        assert!(response.status().is_success());

        let body = response.bytes().await.unwrap();
        let mut body: Value = serde_json::from_slice(&body).unwrap();
        let posts = body.as_array_mut().unwrap();
        assert_eq!(posts.len(), 2);
        for body in posts.iter() {
            assert_eq!(*body.get("title").unwrap(), json!("Post"));
            assert_eq!(*body.get("body").unwrap(), json!("Body"));
            assert_eq!(*body.get("published").unwrap(), json!(false));
        }

        // delete all
        let response = app.delete_all().await;

        assert!(response.status().is_success());

        // all posts should be deleted
        let response = app.list_all().await;

        assert!(response.status().is_success());

        let body = response.bytes().await.unwrap();
        assert_eq!(&body[..], b"[]");
    }

}
