use std::marker::PhantomData;
use serde::Deserialize;

mod servers;
pub mod repositories;
pub use repositories::*;
pub use servers::*;

pub struct Empty;
pub struct Assigned<T>(PhantomData<T>);

pub trait Assignable{}

impl Assignable for Empty{}
impl<T> Assignable for Assigned<T>{}

pub struct CrudRouterBuilder<Server, Repo, Schema, PrimaryKeyType, CreateSchema:Assignable=Empty, UpdateSchema:Assignable=Empty> {
    _marker: PhantomData<(Server, Repo, Schema, PrimaryKeyType, CreateSchema, UpdateSchema)>,
}

impl<Repo> CrudRouterBuilder<Empty, Repo, Empty, Empty, Empty, Empty> {
    pub fn new<Server: ApiServer>() -> CrudRouterBuilder<Assigned<Server>, Repo, Empty, Empty> {
        CrudRouterBuilder {
            _marker: Default::default()
        }
    }
}

impl<Server, Schema, PrimaryKeyType> CrudRouterBuilder<Assigned<Server>, Empty, Schema, PrimaryKeyType> {
    pub fn repository<Repo>(self) -> CrudRouterBuilder<Assigned<Server>, Repo, Schema, PrimaryKeyType>{
        CrudRouterBuilder{
            _marker: Default::default()
        }
    }
}

impl<Server, Repo> CrudRouterBuilder<Assigned<Server>, Repo, Empty, Empty> {
    pub fn schema<Schema, PrimaryKeyType>(self) -> CrudRouterBuilder<Assigned<Server>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>>{
        CrudRouterBuilder{
            _marker: Default::default()
        }
    }
}

impl<Server, Repo, Schema, PrimaryKeyType, UpdateSchema: Assignable> CrudRouterBuilder<Assigned<Server>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, Empty, UpdateSchema> {
    pub fn create_schema<CreateSchema>(self) -> CrudRouterBuilder<Assigned<Server>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, Assigned<CreateSchema>, UpdateSchema>{
        CrudRouterBuilder{
            _marker: Default::default()
        }
    }

}

impl<Server, Repo, Schema, PrimaryKeyType, CreateSchema: Assignable> CrudRouterBuilder<Assigned<Server>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, CreateSchema, Empty> {
    pub fn update_schema<UpdateSchema>(self) -> CrudRouterBuilder<Assigned<Server>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, CreateSchema, Assigned<UpdateSchema>>{
        CrudRouterBuilder{
            _marker: Default::default()
        }
    }

}

#[derive(Deserialize)]
pub struct Pagination{
    skip: Option<u64>,
    limit: Option<u64>,
}
