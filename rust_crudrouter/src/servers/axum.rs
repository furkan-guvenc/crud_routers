use std::sync::Arc;
use axum::extract::{Path, Query, State};
use axum::{Json, Router};
use axum::routing::get;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::Mutex;
use crate::servers::ApiServer;
use crate::{CrudRouterBuilder, Given, NotGiven, OptionalSchema, Pagination};
use crate::repositories::{CreateRepository, ReadDeleteRepository, UpdateRepository};

pub struct AxumServer;

impl ApiServer for AxumServer {}

impl<R, Schema, PrimaryKeyType, CreateSchema, UpdateSchema> CrudRouterBuilder<AxumServer, R, Schema, PrimaryKeyType, CreateSchema, UpdateSchema>
where
    R: ReadDeleteRepository<Schema, PrimaryKeyType> + Send + 'static,
    Schema: Serialize + Send + 'static,
    CreateSchema: OptionalSchema + 'static,
    UpdateSchema: OptionalSchema + 'static,
    PrimaryKeyType: DeserializeOwned + Send + 'static,
{

    async fn list_items_route(
        state: State<Arc<Mutex<R>>>,
        Query(pagination): Query<Pagination>
    ) -> Json<Vec<Schema>>{
        let mut state = state.lock().await;

        R::list_items(&mut state, pagination).await.into()
    }
    async fn get_item_route(
        state: State<Arc<Mutex<R>>>,
        Path(id): Path<PrimaryKeyType>
    ) -> Json<Option<Schema>> {
        let mut state = state.lock().await;

        state.get_item(id).await.into()
    }
    async fn delete_item_route(
        state: State<Arc<Mutex<R>>>,
        Path(id): Path<PrimaryKeyType>
    ) {
        let mut state = state.lock().await;

        state.delete_item(id).await;
    }


    async fn delete_all_items_route(
        state: State<Arc<Mutex<R>>>
    ) -> Json<usize>{
        let mut state = state.lock().await;

        state.delete_all_items().await.into()
    }
}

impl<R, Schema, PrimaryKeyType, CreateSchema, UpdateSchema: OptionalSchema> CrudRouterBuilder<AxumServer, R, Schema, PrimaryKeyType, Given<CreateSchema>, UpdateSchema>
where
    R: CreateRepository<Schema, CreateSchema>,
    Schema: Serialize + Send,
    CreateSchema: DeserializeOwned + Send,
{
    async fn create_item_route(
        state: State<Arc<Mutex<R>>>,
        Json(new_item): Json<CreateSchema>
    ) -> Json<Schema>{
        let mut state = state.lock().await;

        state.create_item(new_item).await.into()
    }

}

impl<R, Schema, PrimaryKeyType, CreateSchema: OptionalSchema, UpdateSchema> CrudRouterBuilder<AxumServer, R, Schema, PrimaryKeyType, CreateSchema, Given<UpdateSchema>>
where
    R: UpdateRepository<Schema, PrimaryKeyType, UpdateSchema>,
    Schema: Serialize + Send,
    UpdateSchema: DeserializeOwned + Send,
{
    async fn update_item_route(
        state: State<Arc<Mutex<R>>>,
        Path(id): Path<PrimaryKeyType>,
        Json(item): Json<UpdateSchema>
    ) -> Json<Schema>{
        let mut state = state.lock().await;

        state.update_item(id, item).await.into()
    }

}


impl<R, Schema, PrimaryKeyType, CreateSchema, UpdateSchema> CrudRouterBuilder<AxumServer, R, Schema, PrimaryKeyType, Given<CreateSchema>, Given<UpdateSchema>>
where
    R: ReadDeleteRepository<Schema, PrimaryKeyType> + CreateRepository<Schema, CreateSchema> + UpdateRepository<Schema, PrimaryKeyType, UpdateSchema> + Send + 'static,
    Schema: Serialize + Send + 'static,
    CreateSchema: DeserializeOwned + Send + 'static,
    UpdateSchema: DeserializeOwned + Send + 'static,
    PrimaryKeyType: DeserializeOwned + Send + 'static,
{
    pub fn build_router(self) -> Router<Arc<Mutex<R>>> {
        Router::new()
            .route("/", get(Self::list_items_route).post(Self::create_item_route).delete(Self::delete_all_items_route))
            .route("/:id", get(Self::get_item_route).put(Self::update_item_route).delete(Self::delete_item_route))
    }
}

impl<R, Schema, PrimaryKeyType, CreateSchema> CrudRouterBuilder<AxumServer, R, Schema, PrimaryKeyType, Given<CreateSchema>>
where
    R: ReadDeleteRepository<Schema, PrimaryKeyType> + CreateRepository<Schema, CreateSchema> + Send + 'static,
    Schema: Serialize + Send + 'static,
    CreateSchema: DeserializeOwned + Send + 'static,
    PrimaryKeyType: DeserializeOwned + Send + 'static,
{
    pub fn build_router(self) -> Router<Arc<Mutex<R>>> {
        Router::new()
            .route("/", get(Self::list_items_route).post(Self::create_item_route).delete(Self::delete_all_items_route))
            .route("/:id", get(Self::get_item_route).delete(Self::delete_item_route))
    }
}

impl<R, Schema, PrimaryKeyType, UpdateSchema> CrudRouterBuilder<AxumServer, R, Schema, PrimaryKeyType, NotGiven, Given<UpdateSchema>>
where
    R: ReadDeleteRepository<Schema, PrimaryKeyType> + UpdateRepository<Schema, PrimaryKeyType, UpdateSchema> + Send + 'static,
    Schema: Serialize + Send + 'static,
    UpdateSchema: DeserializeOwned + Send + 'static,
    PrimaryKeyType: DeserializeOwned + Send + 'static,
{
    pub fn build_router(self) -> Router<Arc<Mutex<R>>> {
        Router::new()
            .route("/", get(Self::list_items_route).delete(Self::delete_all_items_route))
            .route("/:id", get(Self::get_item_route).put(Self::update_item_route).delete(Self::delete_item_route))
    }
}


impl<R, Schema, PrimaryKeyType> CrudRouterBuilder<AxumServer, R, Schema, PrimaryKeyType>
where
    R: ReadDeleteRepository<Schema, PrimaryKeyType> + Send + 'static,
    Schema: Serialize + Send + 'static,
    PrimaryKeyType: DeserializeOwned + Send + 'static,
{
    pub fn build_router(self) -> Router<Arc<Mutex<R>>> {
        Router::new()
            .route("/", get(Self::list_items_route).delete(Self::delete_all_items_route))
            .route("/:id", get(Self::get_item_route).delete(Self::delete_item_route))
    }
}
