use std::env;
use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::Mutex;
use crud_routers::{ActixServer, CrudRouterBuilder, SeaOrmRepository};

pub mod post;

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

async fn establish_connection() -> DatabaseConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(&database_url)
        .await
        .expect("Error connecting to database")
}
