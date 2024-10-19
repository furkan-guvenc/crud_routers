use crate::repositories::{CRUDRepository, ReadDeleteRepository, CreateRepository, UpdateRepository};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityName, EntityTrait, FromQueryResult, IntoActiveModel, ModelTrait, PrimaryKeyTrait, QuerySelect, TryIntoModel};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::Pagination;

pub struct SeaOrmRepository {
    connection: DatabaseConnection
}

impl SeaOrmRepository {
    pub fn new(connection: DatabaseConnection) -> Self{
        Self{
            connection
        }
    }
}

impl CRUDRepository for SeaOrmRepository {}

impl<Schema> ReadDeleteRepository<Schema, <<Schema::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType> for SeaOrmRepository
where
    Schema::Entity: EntityTrait<Model=Schema>,
    Schema: ModelTrait + FromQueryResult + IntoActiveModel<<Schema::Entity as EntityTrait>::ActiveModel> + TryFrom<<Schema::Entity as EntityTrait>::ActiveModel> + DeserializeOwned + Send,
    <Schema::Entity as EntityTrait>::ActiveModel: ActiveModelTrait<Entity=Schema::Entity> + From<Schema> + TryIntoModel<Schema> + Send,
    <<Schema::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: DeserializeOwned + Clone
{
    fn get_table_name() -> String {
        let entity = Schema::Entity::default();
        entity.table_name().to_string()
    }

    async fn list_items(&mut self, pagination: Pagination) -> Vec<Schema> {
        Schema::Entity::find().offset(pagination.skip).limit(pagination.limit).all(&self.connection).await.unwrap()
    }

    async fn get_item(&mut self, id: <<Schema::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType) -> Option<Schema> {
        Schema::Entity::find_by_id(id).one(&self.connection).await.unwrap()
    }

    async fn delete_item(&mut self, id: <<Schema::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType) {
        Schema::Entity::delete_by_id(id).exec(&self.connection).await.unwrap();
    }

    async fn delete_all_items(&mut self) -> usize {
        Schema::Entity::delete_many().exec(&self.connection).await.unwrap().rows_affected as usize
    }
}

impl<Schema, CreateSchema> CreateRepository<Schema, CreateSchema> for SeaOrmRepository
where
    Schema::Entity: EntityTrait<Model=Schema>,
    Schema: ModelTrait + IntoActiveModel<<Schema::Entity as EntityTrait>::ActiveModel> + TryFrom<<Schema::Entity as EntityTrait>::ActiveModel> + DeserializeOwned + Send,
    <Schema::Entity as EntityTrait>::ActiveModel: ActiveModelTrait<Entity=Schema::Entity> + From<Schema> + TryIntoModel<Schema> + Send,
    <<Schema::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: DeserializeOwned + Clone,

    CreateSchema: Serialize + Send
{
    async fn create_item(&mut self, new_item: CreateSchema) -> Schema {
        let new_item_json = serde_json::to_value(new_item).unwrap();

        let active_model = <Schema::Entity as EntityTrait>::ActiveModel::from_json(new_item_json).unwrap();

        active_model.insert(&self.connection).await.unwrap()
    }
}

impl<Schema, UpdateSchema> UpdateRepository<Schema, <<Schema::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType, UpdateSchema> for SeaOrmRepository
where
    Schema::Entity: EntityTrait<Model=Schema>,
    Schema: ModelTrait + FromQueryResult + IntoActiveModel<<Schema::Entity as EntityTrait>::ActiveModel> + TryFrom<<Schema::Entity as EntityTrait>::ActiveModel> + DeserializeOwned + Send,
    <Schema::Entity as EntityTrait>::ActiveModel: ActiveModelTrait<Entity=Schema::Entity> + From<Schema> + TryIntoModel<Schema> + Send,
    <<Schema::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: DeserializeOwned + Clone,

    UpdateSchema: Serialize + Send
{
    async fn update_item(&mut self, id: <<Schema::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType, item: UpdateSchema) -> Schema {
        let item_json = serde_json::to_value(item).unwrap();

        let item = Schema::Entity::find_by_id(id.clone()).one(&self.connection).await.unwrap().unwrap();
        let mut active_model = item.into_active_model();
        active_model.set_from_json(item_json).unwrap();

        active_model.update(&self.connection).await.unwrap()
    }
}
