mod diesel;
mod sea_orm;

pub use sea_orm::SeaOrmRepository;
pub use diesel::DieselRepository;
use crate::{CrudRouterBuilder, NotGiven};
use crate::servers::ApiServer;

pub trait CRUDRepository{
    fn create_router_for<Server: ApiServer>(self) -> CrudRouterBuilder<Server, Self, NotGiven, NotGiven>
    where Self: Sized
    {
        CrudRouterBuilder{
            repo: self,
            _marker: Default::default(),
        }
    }
}

pub trait ReadDeleteRepository<Schema, PrimaryKeyType>: CRUDRepository {
    fn list_items(&mut self) -> impl std::future::Future<Output = Vec<Schema>> + Send;
    fn get_item(&mut self, id: PrimaryKeyType) -> impl std::future::Future<Output = Option<Schema>> + Send;
    fn delete_item(&mut self, id: PrimaryKeyType) -> impl std::future::Future<Output = ()> + Send;
    fn delete_all_items(&mut self) -> impl std::future::Future<Output = usize> + Send;
}

pub trait CreateRepository<Schema, CreateSchema>: CRUDRepository {
    fn create_item(&mut self, new_item: CreateSchema) -> impl std::future::Future<Output = Schema> + Send;
}


pub trait UpdateRepository<Schema, PrimaryKeyType, UpdateSchema>: CRUDRepository {
    fn update_item(&mut self, id: PrimaryKeyType, item: UpdateSchema) -> impl std::future::Future<Output = Schema> + Send;
}
