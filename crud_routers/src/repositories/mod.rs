
#[cfg(feature = "diesel")]
mod diesel;
#[cfg(feature = "sea-orm")]
mod sea_orm;

#[cfg(feature = "sea-orm")]
pub use sea_orm::SeaOrmRepository;
#[cfg(feature = "diesel")]
pub use diesel::DieselRepository;

use crate::Pagination;

pub trait CRUDRepository{}

pub trait ReadDeleteRepository<Schema, PrimaryKeyType>: CRUDRepository {
    fn get_table_name() -> String;
    fn list_items(&mut self, pagination: Pagination) -> impl std::future::Future<Output = Vec<Schema>> + Send;
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
