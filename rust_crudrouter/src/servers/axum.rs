use std::sync::Arc;
use axum::extract::{Path, Query, State};
use axum::{routing, Json, Router};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::Mutex;
use crate::servers::ApiServer;
use crate::{CrudRouterBuilder, Assigned, Empty, Assignable, Pagination};
use crate::repositories::{CreateRepository, ReadDeleteRepository, UpdateRepository};

pub struct AxumServer;

impl ApiServer for AxumServer {
    fn get_id_path(prefix: &str) -> String {
        format!("/{}/:id", prefix)
    }
}

impl<R, Schema, PrimaryKeyType, CreateSchema, UpdateSchema> CrudRouterBuilder<'_, Assigned<AxumServer>, R, Assigned<Schema>, Assigned<PrimaryKeyType>, CreateSchema, UpdateSchema>
where
    R: ReadDeleteRepository<Schema, PrimaryKeyType> + Send + 'static,
    Schema: Serialize + Send + 'static,
    CreateSchema: Assignable + 'static,
    UpdateSchema: Assignable + 'static,
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

impl<R, Schema, PrimaryKeyType, CreateSchema, UpdateSchema: Assignable> CrudRouterBuilder<'_, Assigned<AxumServer>, R, Assigned<Schema>, Assigned<PrimaryKeyType>, Assigned<CreateSchema>, UpdateSchema>
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

impl<R, Schema, PrimaryKeyType, CreateSchema: Assignable, UpdateSchema> CrudRouterBuilder<'_, Assigned<AxumServer>, R, Assigned<Schema>, Assigned<PrimaryKeyType>, CreateSchema, Assigned<UpdateSchema>>
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


impl<R, Schema, PrimaryKeyType, CreateSchema, UpdateSchema> CrudRouterBuilder<'_, Assigned<AxumServer>, R, Assigned<Schema>, Assigned<PrimaryKeyType>, Assigned<CreateSchema>, Assigned<UpdateSchema>>
where
    R: ReadDeleteRepository<Schema, PrimaryKeyType> + CreateRepository<Schema, CreateSchema> + UpdateRepository<Schema, PrimaryKeyType, UpdateSchema> + Send + 'static,
    Schema: Serialize + Send + 'static,
    CreateSchema: DeserializeOwned + Send + 'static,
    UpdateSchema: DeserializeOwned + Send + 'static,
    PrimaryKeyType: DeserializeOwned + Send + 'static,
{
    pub fn build_router(self) -> Router<Arc<Mutex<R>>> {
        let mut r = Router::new();
        let prefix = self.get_prefix();
        let path = AxumServer::get_path(&prefix);
        let id_path = AxumServer::get_id_path(&prefix);

        if !self.list_items_route_disabled {
            r = r.route(&path, routing::get(Self::list_items_route))
        }
        if !self.create_item_route_disabled {
            r = r.route(&path, routing::post(Self::create_item_route))
        }
        if !self.delete_all_items_route_disabled {
            r = r.route(&path, routing::delete(Self::delete_all_items_route))
        }
        if !self.get_item_route_disabled {
            r = r.route(&id_path, routing::get(Self::get_item_route))
        }
        if !self.update_item_route_disabled {
            r = r.route(&id_path, routing::put(Self::update_item_route))
        }
        if !self.delete_item_route_disabled {
            r = r.route(&id_path, routing::delete(Self::delete_item_route))
        }

        r
    }
}

impl<R, Schema, PrimaryKeyType, CreateSchema> CrudRouterBuilder<'_, Assigned<AxumServer>, R, Assigned<Schema>, Assigned<PrimaryKeyType>, Assigned<CreateSchema>, Empty>
where
    R: ReadDeleteRepository<Schema, PrimaryKeyType> + CreateRepository<Schema, CreateSchema> + Send + 'static,
    Schema: Serialize + Send + 'static,
    CreateSchema: DeserializeOwned + Send + 'static,
    PrimaryKeyType: DeserializeOwned + Send + 'static,
{
    pub fn build_router(self) -> Router<Arc<Mutex<R>>> {
        let mut r = Router::new();
        let prefix = self.get_prefix();
        let path = AxumServer::get_path(&prefix);
        let id_path = AxumServer::get_id_path(&prefix);

        if !self.list_items_route_disabled {
            r = r.route(&path, routing::get(Self::list_items_route))
        }
        if !self.create_item_route_disabled {
            r = r.route(&path, routing::post(Self::create_item_route))
        }
        if !self.delete_all_items_route_disabled {
            r = r.route(&path, routing::delete(Self::delete_all_items_route))
        }
        if !self.get_item_route_disabled {
            r = r.route(&id_path, routing::get(Self::get_item_route))
        }
        if !self.delete_item_route_disabled {
            r = r.route(&id_path, routing::delete(Self::delete_item_route))
        }

        r
    }
}

impl<R, Schema, PrimaryKeyType, UpdateSchema> CrudRouterBuilder<'_, Assigned<AxumServer>, R, Assigned<Schema>, Assigned<PrimaryKeyType>, Empty, Assigned<UpdateSchema>>
where
    R: ReadDeleteRepository<Schema, PrimaryKeyType> + UpdateRepository<Schema, PrimaryKeyType, UpdateSchema> + Send + 'static,
    Schema: Serialize + Send + 'static,
    UpdateSchema: DeserializeOwned + Send + 'static,
    PrimaryKeyType: DeserializeOwned + Send + 'static,
{
    pub fn build_router(self) -> Router<Arc<Mutex<R>>> {
        let mut r = Router::new();
        let prefix = self.get_prefix();
        let path = AxumServer::get_path(&prefix);
        let id_path = AxumServer::get_id_path(&prefix);

        if !self.list_items_route_disabled {
            r = r.route(&path, routing::get(Self::list_items_route))
        }
        if !self.delete_all_items_route_disabled {
            r = r.route(&path, routing::delete(Self::delete_all_items_route))
        }
        if !self.get_item_route_disabled {
            r = r.route(&id_path, routing::get(Self::get_item_route))
        }
        if !self.update_item_route_disabled {
            r = r.route(&id_path, routing::put(Self::update_item_route))
        }
        if !self.delete_item_route_disabled {
            r = r.route(&id_path, routing::delete(Self::delete_item_route))
        }

        r
    }
}


impl<R, Schema, PrimaryKeyType> CrudRouterBuilder<'_, Assigned<AxumServer>, R, Assigned<Schema>, Assigned<PrimaryKeyType>, Empty, Empty>
where
    R: ReadDeleteRepository<Schema, PrimaryKeyType> + Send + 'static,
    Schema: Serialize + Send + 'static,
    PrimaryKeyType: DeserializeOwned + Send + 'static,
{
    pub fn build_router(self) -> Router<Arc<Mutex<R>>> {
        let mut r = Router::new();
        let prefix = self.get_prefix();
        let path = AxumServer::get_path(&prefix);
        let id_path = AxumServer::get_id_path(&prefix);

        if !self.list_items_route_disabled {
            r = r.route(&path, routing::get(Self::list_items_route))
        }
        if !self.delete_all_items_route_disabled {
            r = r.route(&path, routing::delete(Self::delete_all_items_route))
        }
        if !self.get_item_route_disabled {
            r = r.route(&id_path, routing::get(Self::get_item_route))
        }
        if !self.delete_item_route_disabled {
            r = r.route(&id_path, routing::delete(Self::delete_item_route))
        }

        r
    }
}
