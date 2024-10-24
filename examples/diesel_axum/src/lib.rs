pub mod models;
pub mod schema;

use std::env;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use std::sync::Arc;
use axum::Router;
use axum::serve::Serve;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use utoipa::openapi::{InfoBuilder, OpenApiBuilder};
use utoipa_swagger_ui::SwaggerUi;
use crud_routers::{AxumServer, CrudRouterBuilder, DieselRepository};
use crate::models::{NewPost, Post, PostForm};
use crate::schema::posts;

pub fn run(listener: TcpListener) -> Serve<Router, Router> {
    let mut openapi = OpenApiBuilder::new()
        .info(InfoBuilder::new().title("Diesel Axum example").build())
        .build();

    let connection = establish_connection();
    let shared_state = Arc::new(Mutex::new(
        DieselRepository::new(connection, posts::table)
    ));

    let router = CrudRouterBuilder::new::<AxumServer>()
        .schema::<Post, i32>()
        .create_schema::<NewPost>()
        .update_schema::<PostForm>()
        .prefix("base/api")
        .build_openapi(&mut openapi)
        .build_router()
        .with_state(shared_state)
        .merge(SwaggerUi::new("/docs/swagger/").url("/api-docs/openapi.json", openapi));

    axum::serve(listener, router)
}


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

