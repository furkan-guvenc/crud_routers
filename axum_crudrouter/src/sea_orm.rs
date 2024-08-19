use crate::CRUDGenerator;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, PrimaryKeyArity, PrimaryKeyTrait, TryIntoModel};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SeaOrmCRUDRouter<Entity>
where
    Entity: EntityTrait,
    Entity::Model: ModelTrait<Entity=Entity> + Serialize + IntoActiveModel<Entity::ActiveModel> + TryFrom<Entity::ActiveModel> + DeserializeOwned + Send,
    Entity::ActiveModel: ActiveModelTrait<Entity=Entity> + From<Entity::Model> + TryIntoModel<Entity::Model> + Send,
    <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType: DeserializeOwned + Clone,
{
    connection: DatabaseConnection,
    _entity: PhantomData<Entity>
}



impl<Entity> SeaOrmCRUDRouter<Entity>
where
    Entity: EntityTrait,
    Entity::Model: ModelTrait<Entity=Entity> + Serialize + IntoActiveModel<Entity::ActiveModel> + TryFrom<Entity::ActiveModel> + DeserializeOwned + Send,
    Entity::ActiveModel: ActiveModelTrait<Entity=Entity> + From<Entity::Model> + TryIntoModel<Entity::Model> + Send,
    <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType: DeserializeOwned + Clone,
{

    pub fn build<Schema: ModelTrait<Entity=Entity>>(connection: DatabaseConnection) -> Router {
        if <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType::ARITY != 1 {
            panic!("SeaOrmCRUDRouter library doesn't support composite primary keys");
        }

        let shared_state = Arc::new(Mutex::new(Self {
            connection,
            _entity: PhantomData::<Schema::Entity>,
        }));

        Router::new()
            .route("/", get(Self::list_items_route).post(Self::create_item_route).delete(Self::delete_all_items_route))
            .route("/:id", get(Self::get_item_route).put(Self::update_item_route).delete(Self::delete_item_route))
            .with_state(shared_state)
    }
}

impl<Entity> CRUDGenerator<Entity::Model, <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType> for SeaOrmCRUDRouter<Entity>
where
    Entity: EntityTrait,
    Entity::Model: ModelTrait<Entity=Entity> + Serialize + IntoActiveModel<Entity::ActiveModel> + TryFrom<Entity::ActiveModel> + DeserializeOwned + Send,
    Entity::ActiveModel: ActiveModelTrait<Entity=Entity> + From<Entity::Model> + TryIntoModel<Entity::Model> + Send,
    <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType: DeserializeOwned + Clone,
{
    async fn list_items_route(state: State<Arc<Mutex<Self>>>) -> Json<Vec<Entity::Model>>
    {
        let state = state.lock().await;

        Entity::find().all(&state.connection).await.unwrap().into()
    }

    async fn get_item_route(
        State(state): State<Arc<Mutex<Self>>>,
        Path(id): Path<<Entity::PrimaryKey as PrimaryKeyTrait>::ValueType>
    ) -> Json<Option<Entity::Model>> {
        let state = state.lock().await;

        Entity::find_by_id(id).one(&state.connection).await.unwrap().into()

    }

    async fn create_item_route(
        State(state): State<Arc<Mutex<Self>>>,
        Json(new_item_json): Json<serde_json::Value>
    ) -> Json<Entity::Model> {
        let state = state.lock().await;

        let active_model = Entity::ActiveModel::from_json(new_item_json).unwrap();

        active_model.insert(&state.connection).await.unwrap().into()
    }

    async fn update_item_route(
        State(state): State<Arc<Mutex<Self>>>,
        Path(id): Path<<Entity::PrimaryKey as PrimaryKeyTrait>::ValueType>,
        Json(item_json): Json<serde_json::Value>
    ) -> Json<Entity::Model> {
        let state = state.lock().await;

        let item = Entity::find_by_id(id.clone()).one(&state.connection).await.unwrap().unwrap();
        let mut active_model = item.into_active_model();
        active_model.set_from_json(item_json).unwrap();

        active_model.update(&state.connection).await.unwrap().into()
    }


    async fn delete_item_route(
        State(state): State<Arc<Mutex<Self>>>,
        Path(id): Path<<Entity::PrimaryKey as PrimaryKeyTrait>::ValueType>
    ) {
        let state = state.lock().await;

        Entity::delete_by_id(id).exec(&state.connection).await.unwrap();
    }

    async fn delete_all_items_route(
        State(state): State<Arc<Mutex<Self>>>
    ) -> Json<usize> {
        let state = state.lock().await;

        (Entity::delete_many().exec(&state.connection).await.unwrap().rows_affected as usize).into()
    }
}