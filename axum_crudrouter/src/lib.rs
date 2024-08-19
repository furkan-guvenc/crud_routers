use axum::extract::{Path, State};
use axum::Json;
use std::sync::Arc;
use tokio::sync::Mutex;

mod diesel;
mod sea_orm;

pub use sea_orm::SeaOrmCRUDRouter;

trait CRUDGenerator<Schema, PrimaryKeyType> {

    async fn list_items_route(
        state: State<Arc<Mutex<Self>>>
    ) -> Json<Vec<Schema>>;

    async fn get_item_route(
        state: State<Arc<Mutex<Self>>>,
        id: Path<PrimaryKeyType>
    ) -> Json<Option<Schema>>;

    async fn create_item_route(
        state: State<Arc<Mutex<Self>>>,
        new_item_json: Json<serde_json::Value>
    ) -> Json<Schema>;

    async fn update_item_route(
        state: State<Arc<Mutex<Self>>>,
        id: Path<PrimaryKeyType>,
        item_json: Json<serde_json::Value>
    ) -> Json<Schema>;

    async fn delete_item_route(
        state: State<Arc<Mutex<Self>>>,
        id: Path<PrimaryKeyType>
    );

    async fn delete_all_items_route(
        state: State<Arc<Mutex<Self>>>
    ) -> Json<usize>;
}
