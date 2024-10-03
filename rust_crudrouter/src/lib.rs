use std::marker::PhantomData;

mod servers;
pub mod repositories;
pub use repositories::*;
pub use servers::*;

pub struct NotGiven;
pub struct Given<T>{
    _marker: PhantomData<T>
}

pub trait OptionalSchema{}

impl OptionalSchema for NotGiven{}
impl<T> OptionalSchema for Given<T>{}

pub struct CrudRouterBuilder<Server, Repo, Schema, PrimaryKeyType, CreateSchema:OptionalSchema=NotGiven, UpdateSchema:OptionalSchema=NotGiven> {
    _marker: PhantomData<(Server, Repo, Schema, PrimaryKeyType, CreateSchema, UpdateSchema)>,
}

impl<Repo> CrudRouterBuilder<NotGiven, Repo, NotGiven, NotGiven, NotGiven, NotGiven> {
    pub fn new<Server: ApiServer>() -> CrudRouterBuilder<Server, Repo, NotGiven, NotGiven> {
        CrudRouterBuilder {
            _marker: Default::default()
        }
    }
}

impl<Server, Schema, PrimaryKeyType> CrudRouterBuilder<Server, NotGiven, Schema, PrimaryKeyType> {
    pub fn repository<Repo>(self) -> CrudRouterBuilder<Server, Repo, Schema, PrimaryKeyType>{
        CrudRouterBuilder{
            _marker: Default::default()
        }
    }
}

impl<Server, Repo> CrudRouterBuilder<Server, Repo, NotGiven, NotGiven> {
    pub fn schema<Schema, PrimaryKeyType>(self) -> CrudRouterBuilder<Server, Repo, Schema, PrimaryKeyType>{
        CrudRouterBuilder{
            _marker: Default::default()
        }
    }
}

impl<Server, Repo, Schema, PrimaryKeyType, UpdateSchema: OptionalSchema> CrudRouterBuilder<Server, Repo, Schema, PrimaryKeyType, NotGiven, UpdateSchema> {
    pub fn create_schema<CreateSchema>(self) -> CrudRouterBuilder<Server, Repo, Schema, PrimaryKeyType, Given<CreateSchema>, UpdateSchema>{
        CrudRouterBuilder{
            _marker: Default::default()
        }
    }

}

impl<Server, Repo, Schema, PrimaryKeyType, CreateSchema: OptionalSchema> CrudRouterBuilder<Server, Repo, Schema, PrimaryKeyType, CreateSchema, NotGiven> {
    pub fn update_schema<UpdateSchema>(self) -> CrudRouterBuilder<Server, Repo, Schema, PrimaryKeyType, CreateSchema, Given<UpdateSchema>>{
        CrudRouterBuilder{
            _marker: Default::default()
        }
    }

}
