use actix_web::{web, HttpResponse, Scope};
use actix_web::web::{Data, Json, Path, Query};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::Mutex;
use crate::servers::ApiServer;
use crate::{CrudRouterBuilder, Assigned, Empty, Assignable, Pagination};
use crate::repositories::{CreateRepository, ReadDeleteRepository, UpdateRepository};

pub struct ActixServer {}

impl ApiServer for ActixServer {}

impl<R, Schema, PrimaryKeyType, CreateSchema, UpdateSchema> CrudRouterBuilder<Assigned<ActixServer>, R, Assigned<Schema>, Assigned<PrimaryKeyType>, CreateSchema, UpdateSchema>
where
    R: ReadDeleteRepository<Schema, PrimaryKeyType> + Send + 'static,
    Schema: Serialize + Send + 'static,
    CreateSchema: Assignable + 'static,
    UpdateSchema: Assignable + 'static,
    PrimaryKeyType: DeserializeOwned + Send + 'static,
{

    async fn list_items_route(
        state: Data<Mutex<R>>,
        pagination: Query<Pagination>
    ) -> Json<Vec<Schema>>{
        let mut state = state.lock().await;

        Json(R::list_items(&mut state, pagination.into_inner()).await)
    }
    async fn get_item_route(
        state: Data<Mutex<R>>,
        id: Path<PrimaryKeyType>
    ) -> Json<Option<Schema>> {
        let mut state = state.lock().await;

        Json(state.get_item(id.into_inner()).await)
    }
    async fn delete_item_route(
        state: Data<Mutex<R>>,
        id: Path<PrimaryKeyType>
    ) -> HttpResponse {
        let mut state = state.lock().await;

        state.delete_item(id.into_inner()).await;
        HttpResponse::Ok().finish()
    }


    async fn delete_all_items_route(
        state: Data<Mutex<R>>
    ) -> Json<usize>{
        let mut state = state.lock().await;

        Json(state.delete_all_items().await)
    }
}

impl<R, Schema, PrimaryKeyType, CreateSchema, UpdateSchema: Assignable> CrudRouterBuilder<Assigned<ActixServer>, R, Assigned<Schema>, Assigned<PrimaryKeyType>, Assigned<CreateSchema>, UpdateSchema>
where
    R: CreateRepository<Schema, CreateSchema>,
    Schema: Serialize + Send,
    CreateSchema: DeserializeOwned + Send,
{
    async fn create_item_route(
        state: Data<Mutex<R>>,
        Json(new_item): Json<CreateSchema>
    ) -> Json<Schema>{
        let mut state = state.lock().await;

        Json(state.create_item(new_item).await)
    }

}

impl<R, Schema, PrimaryKeyType, CreateSchema: Assignable, UpdateSchema> CrudRouterBuilder<Assigned<ActixServer>, R, Assigned<Schema>, Assigned<PrimaryKeyType>, CreateSchema, Assigned<UpdateSchema>>
where
    R: UpdateRepository<Schema, PrimaryKeyType, UpdateSchema>,
    Schema: Serialize + Send,
    UpdateSchema: DeserializeOwned + Send,
{
    async fn update_item_route(
        state: Data<Mutex<R>>,
        id: Path<PrimaryKeyType>,
        Json(item): Json<UpdateSchema>
    ) -> Json<Schema>{
        let mut state = state.lock().await;

        Json(state.update_item(id.into_inner(), item).await)
    }

}


impl<R, Schema, PrimaryKeyType, CreateSchema, UpdateSchema> CrudRouterBuilder<Assigned<ActixServer>, R, Assigned<Schema>, Assigned<PrimaryKeyType>, Assigned<CreateSchema>, Assigned<UpdateSchema>>
where
    R: ReadDeleteRepository<Schema, PrimaryKeyType> + CreateRepository<Schema, CreateSchema> + UpdateRepository<Schema, PrimaryKeyType, UpdateSchema> + Send + 'static,
    Schema: Serialize + Send + 'static,
    CreateSchema: DeserializeOwned + Send + 'static,
    UpdateSchema: DeserializeOwned + Send + 'static,
    PrimaryKeyType: DeserializeOwned + Send + 'static,
{
    pub fn build_router(self) -> Scope {
        web::scope("")
            .route("/", web::get().to(Self::list_items_route))
            .route("/", web::post().to(Self::create_item_route))
            .route("/", web::delete().to(Self::delete_all_items_route))
            .route("/{id}", web::get().to(Self::get_item_route))
            .route("/{id}", web::put().to(Self::update_item_route))
            .route("/{id}", web::delete().to(Self::delete_item_route))
    }
}

impl<R, Schema, PrimaryKeyType, CreateSchema> CrudRouterBuilder<Assigned<ActixServer>, R, Assigned<Schema>, Assigned<PrimaryKeyType>, Assigned<CreateSchema>>
where
    R: ReadDeleteRepository<Schema, PrimaryKeyType> + CreateRepository<Schema, CreateSchema> + Send + 'static,
    Schema: Serialize + Send + 'static,
    CreateSchema: DeserializeOwned + Send + 'static,
    PrimaryKeyType: DeserializeOwned + Send + 'static,
{
    pub fn build_router(self) -> Scope {
        web::scope("")
            .route("/", web::get().to(Self::list_items_route))
            .route("/", web::post().to(Self::create_item_route))
            .route("/", web::delete().to(Self::delete_all_items_route))
            .route("/{id}", web::get().to(Self::get_item_route))
            .route("/{id}", web::delete().to(Self::delete_item_route))
    }
}

impl<R, Schema, PrimaryKeyType, UpdateSchema> CrudRouterBuilder<Assigned<ActixServer>, R, Assigned<Schema>, Assigned<PrimaryKeyType>, Empty, Assigned<UpdateSchema>>
where
    R: ReadDeleteRepository<Schema, PrimaryKeyType> + UpdateRepository<Schema, PrimaryKeyType, UpdateSchema> + Send + 'static,
    Schema: Serialize + Send + 'static,
    UpdateSchema: DeserializeOwned + Send + 'static,
    PrimaryKeyType: DeserializeOwned + Send + 'static,
{
    pub fn build_router(self) -> Scope {
        web::scope("")
            .route("/", web::get().to(Self::list_items_route))
            .route("/", web::delete().to(Self::delete_all_items_route))
            .route("/{id}", web::get().to(Self::get_item_route))
            .route("/{id}", web::put().to(Self::update_item_route))
            .route("/{id}", web::delete().to(Self::delete_item_route))
    }
}


impl<R, Schema, PrimaryKeyType> CrudRouterBuilder<Assigned<ActixServer>, R, Assigned<Schema>, Assigned<PrimaryKeyType>>
where
    R: ReadDeleteRepository<Schema, PrimaryKeyType> + Send + 'static,
    Schema: Serialize + Send + 'static,
    PrimaryKeyType: DeserializeOwned + Send + 'static,
{
    pub fn build_router(self) -> Scope {
        web::scope("")
            .route("/", web::get().to(Self::list_items_route))
            .route("/", web::delete().to(Self::delete_all_items_route))
            .route("/{id}", web::get().to(Self::get_item_route))
            .route("/{id}", web::delete().to(Self::delete_item_route))
    }
}
