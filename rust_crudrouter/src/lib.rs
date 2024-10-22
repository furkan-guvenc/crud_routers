use std::marker::PhantomData;
use serde::Deserialize;

mod servers;
mod repositories;
#[cfg(feature = "openapi")]
mod openapi;

pub use repositories::*;
pub use servers::*;

pub struct Empty;
pub struct Assigned<T>(PhantomData<T>);

pub trait Assignable{
    const IS_ASSIGNED: bool;
}

impl Assignable for Empty{
    const IS_ASSIGNED: bool = false;
}
impl<T> Assignable for Assigned<T>{
    const IS_ASSIGNED: bool = true;
}

pub struct CrudRouterBuilder<'a, Server: Assignable, Repo, Schema: Assignable, PrimaryKeyType: Assignable, CreateSchema:Assignable, UpdateSchema:Assignable> {
    prefix: Option<&'a str>,
    tag: Option<&'a str>,
    list_items_route_disabled: bool,
    get_item_route_disabled: bool,
    delete_item_route_disabled: bool,
    delete_all_items_route_disabled: bool,
    create_item_route_disabled: bool,
    update_item_route_disabled: bool,
    _marker: PhantomData<(Server, Repo, Schema, PrimaryKeyType, CreateSchema, UpdateSchema)>,
}

impl<'a, Repo> CrudRouterBuilder<'a, Empty, Repo, Empty, Empty, Empty, Empty> {
    pub fn new<Server: ApiServer>() -> CrudRouterBuilder<'a, Assigned<Server>, Repo, Empty, Empty, Empty, Empty> {
        CrudRouterBuilder {
            prefix: None,
            tag: None,
            list_items_route_disabled: false,
            get_item_route_disabled: false,
            delete_item_route_disabled: false,
            delete_all_items_route_disabled: false,
            create_item_route_disabled: false,
            update_item_route_disabled: false,
            _marker: Default::default()
        }
    }
}

impl <'a, Server, Repo, Schema: Assignable, PrimaryKeyType: Assignable, CreateSchema:Assignable, UpdateSchema:Assignable> CrudRouterBuilder<'a, Assigned<Server>, Repo, Schema, PrimaryKeyType, CreateSchema, UpdateSchema> {
    pub fn prefix(self, prefix: &'a str) -> Self{
        CrudRouterBuilder{
            prefix: Some(prefix),
            tag: self.tag,
            _marker: Default::default(),
            list_items_route_disabled: self.list_items_route_disabled,
            get_item_route_disabled: self.get_item_route_disabled,
            delete_item_route_disabled: self.delete_item_route_disabled,
            delete_all_items_route_disabled: self.delete_all_items_route_disabled,
            create_item_route_disabled: self.create_item_route_disabled,
            update_item_route_disabled: self.update_item_route_disabled,
        }
    }

    #[cfg(feature = "openapi")]
    pub fn tag(self, tag: &'a str) -> Self{
        CrudRouterBuilder{
            prefix: self.prefix,
            tag: Some(tag),
            _marker: Default::default(),
            list_items_route_disabled: self.list_items_route_disabled,
            get_item_route_disabled: self.get_item_route_disabled,
            delete_item_route_disabled: self.delete_item_route_disabled,
            delete_all_items_route_disabled: self.delete_all_items_route_disabled,
            create_item_route_disabled: self.create_item_route_disabled,
            update_item_route_disabled: self.update_item_route_disabled,
        }
    }
}

impl<'a, Server, Schema: Assignable, PrimaryKeyType: Assignable> CrudRouterBuilder<'a, Assigned<Server>, Empty, Schema, PrimaryKeyType, Empty, Empty> {
    pub fn repository<Repo: CRUDRepository>(self) -> CrudRouterBuilder<'a, Assigned<Server>, Repo, Schema, PrimaryKeyType, Empty, Empty>{
        CrudRouterBuilder{
            prefix: self.prefix,
            tag: self.tag,
            _marker: Default::default(),
            list_items_route_disabled: self.list_items_route_disabled,
            get_item_route_disabled: self.get_item_route_disabled,
            delete_item_route_disabled: self.delete_item_route_disabled,
            delete_all_items_route_disabled: self.delete_all_items_route_disabled,
            create_item_route_disabled: self.create_item_route_disabled,
            update_item_route_disabled: self.update_item_route_disabled,
        }
    }
}

impl<'a, Server, Repo> CrudRouterBuilder<'a, Assigned<Server>, Repo, Empty, Empty, Empty, Empty> {
    #[cfg(not(feature = "openapi"))]
    pub fn schema<Schema, PrimaryKeyType>(self) -> CrudRouterBuilder<'a, Assigned<Server>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, Empty, Empty>{
        CrudRouterBuilder{
            prefix: self.prefix,
            tag: self.tag,
            _marker: Default::default(),
            list_items_route_disabled: self.list_items_route_disabled,
            get_item_route_disabled: self.get_item_route_disabled,
            delete_item_route_disabled: self.delete_item_route_disabled,
            delete_all_items_route_disabled: self.delete_all_items_route_disabled,
            create_item_route_disabled: self.create_item_route_disabled,
            update_item_route_disabled: self.update_item_route_disabled,
        }
    }

    #[cfg(feature = "openapi")]
    pub fn schema<Schema: utoipa::ToSchema, PrimaryKeyType>(self) -> CrudRouterBuilder<'a, Assigned<Server>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, Empty, Empty>{
        CrudRouterBuilder{
            prefix: self.prefix,
            tag: self.tag,
            _marker: Default::default(),
            list_items_route_disabled: self.list_items_route_disabled,
            get_item_route_disabled: self.get_item_route_disabled,
            delete_item_route_disabled: self.delete_item_route_disabled,
            delete_all_items_route_disabled: self.delete_all_items_route_disabled,
            create_item_route_disabled: self.create_item_route_disabled,
            update_item_route_disabled: self.update_item_route_disabled,
        }
    }
}

impl<'a, Server, Repo, Schema, PrimaryKeyType, UpdateSchema: Assignable> CrudRouterBuilder<'a, Assigned<Server>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, Empty, UpdateSchema> {
    #[cfg(not(feature = "openapi"))]
    pub fn create_schema<CreateSchema>(self) -> CrudRouterBuilder<'a, Assigned<Server>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, Assigned<CreateSchema>, UpdateSchema>{
        CrudRouterBuilder{
            prefix: self.prefix,
            tag: self.tag,
            _marker: Default::default(),
            list_items_route_disabled: self.list_items_route_disabled,
            get_item_route_disabled: self.get_item_route_disabled,
            delete_item_route_disabled: self.delete_item_route_disabled,
            delete_all_items_route_disabled: self.delete_all_items_route_disabled,
            create_item_route_disabled: self.create_item_route_disabled,
            update_item_route_disabled: self.update_item_route_disabled,
        }
    }

    #[cfg(feature = "openapi")]
    pub fn create_schema<CreateSchema: utoipa::ToSchema>(self) -> CrudRouterBuilder<'a, Assigned<Server>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, Assigned<CreateSchema>, UpdateSchema>{
        CrudRouterBuilder{
            prefix: self.prefix,
            tag: self.tag,
            _marker: Default::default(),
            list_items_route_disabled: self.list_items_route_disabled,
            get_item_route_disabled: self.get_item_route_disabled,
            delete_item_route_disabled: self.delete_item_route_disabled,
            delete_all_items_route_disabled: self.delete_all_items_route_disabled,
            create_item_route_disabled: self.create_item_route_disabled,
            update_item_route_disabled: self.update_item_route_disabled,
        }
    }
}

impl<Server: Assignable, Repo: ReadDeleteRepository<Schema, PrimaryKeyType>, Schema, PrimaryKeyType, CreateSchema: Assignable, UpdateSchema: Assignable> CrudRouterBuilder<'_, Server, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, CreateSchema, UpdateSchema> {
    fn get_prefix(&self) -> &str{
        if let Some(prefix) = self.prefix {
            prefix
        } else {
            Repo::get_table_name().leak()
        }
    }
}

impl<'a, Server, Repo, Schema, PrimaryKeyType, CreateSchema: Assignable> CrudRouterBuilder<'a, Assigned<Server>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, CreateSchema, Empty> {
    #[cfg(not(feature = "openapi"))]
    pub fn update_schema<UpdateSchema>(self) -> CrudRouterBuilder<'a, Assigned<Server>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, CreateSchema, Assigned<UpdateSchema>>{
        CrudRouterBuilder{
            prefix: self.prefix,
            tag: self.tag,
            _marker: Default::default(),
            list_items_route_disabled: self.list_items_route_disabled,
            get_item_route_disabled: self.get_item_route_disabled,
            delete_item_route_disabled: self.delete_item_route_disabled,
            delete_all_items_route_disabled: self.delete_all_items_route_disabled,
            create_item_route_disabled: self.create_item_route_disabled,
            update_item_route_disabled: self.update_item_route_disabled,
        }
    }

    #[cfg(feature = "openapi")]
    pub fn update_schema<UpdateSchema: utoipa::ToSchema>(self) -> CrudRouterBuilder<'a, Assigned<Server>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, CreateSchema, Assigned<UpdateSchema>>{
        CrudRouterBuilder{
            prefix: self.prefix,
            tag: self.tag,
            _marker: Default::default(),
            list_items_route_disabled: self.list_items_route_disabled,
            get_item_route_disabled: self.get_item_route_disabled,
            delete_item_route_disabled: self.delete_item_route_disabled,
            delete_all_items_route_disabled: self.delete_all_items_route_disabled,
            create_item_route_disabled: self.create_item_route_disabled,
            update_item_route_disabled: self.update_item_route_disabled,
        }
    }
}

impl<Server, Repo, Schema, PrimaryKeyType, CreateSchema: Assignable, UpdateSchema: Assignable> CrudRouterBuilder<'_, Assigned<Server>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, CreateSchema, UpdateSchema> {
    pub fn disable_list_items_route(self) -> Self{
        Self {
            list_items_route_disabled: true,
            ..self
        }
    }

    pub fn disable_get_item_route(self) -> Self{
        Self {
            get_item_route_disabled: true,
            ..self
        }
    }

    pub fn disable_delete_item_route(self) -> Self{
        Self {
            delete_item_route_disabled: true,
            ..self
        }
    }

    pub fn disable_delete_all_items_route(self) -> Self{
        Self {
            delete_all_items_route_disabled: true,
            ..self
        }
    }
}

impl<Server, Repo, Schema, PrimaryKeyType, CreateSchema, UpdateSchema: Assignable> CrudRouterBuilder<'_, Assigned<Server>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, Assigned<CreateSchema>, UpdateSchema> {
    pub fn disable_create_item_route(self) -> Self{
        Self {
            create_item_route_disabled: true,
            ..self
        }
    }
}

impl<Server, Repo, Schema, PrimaryKeyType, CreateSchema: Assignable, UpdateSchema> CrudRouterBuilder<'_, Assigned<Server>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, CreateSchema, Assigned<UpdateSchema>> {
    pub fn disable_update_item_route(self) -> Self{
        Self {
            update_item_route_disabled: true,
            ..self
        }
    }
}

#[derive(Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::IntoParams))]
pub struct Pagination{
    skip: Option<u64>,
    limit: Option<u64>,
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    struct Schema;
    struct CreateSchema;
    struct UpdateSchema;
    struct PrimaryKeyType;
    struct Repo;
    impl ReadDeleteRepository<Schema, PrimaryKeyType> for Repo {
        fn get_table_name() -> String {
            String::from("test_table_name")
        }

        async fn list_items(&mut self, _pagination: Pagination) -> Vec<Schema> {
            unimplemented!()
        }

        async fn get_item(&mut self, _id: PrimaryKeyType) -> Option<Schema> {
            unimplemented!()
        }

        async fn delete_item(&mut self, _id: PrimaryKeyType) {
            unimplemented!()
        }

        async fn delete_all_items(&mut self) -> usize {
            unimplemented!()
        }
    }
    impl CRUDRepository for Repo {}
    struct TestServer;
    impl ApiServer for TestServer {
        fn get_id_path(prefix: &str) -> String {
            unimplemented!()
        }
    }
    impl CrudRouterBuilder<'_, Assigned<TestServer>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, Assigned<CreateSchema>, Assigned<UpdateSchema>>
    {
        pub fn test_get_prefix(&self) -> &str {
            self.get_prefix()
        }
    }

    #[test]
    fn test_all_routes_enabled() {
        let b = CrudRouterBuilder::new::<TestServer>()
            .repository::<Repo>();
        assert!(!b.list_items_route_disabled);
        assert!(!b.get_item_route_disabled);
        assert!(!b.delete_item_route_disabled);
        assert!(!b.delete_all_items_route_disabled);
        assert!(!b.create_item_route_disabled);
        assert!(!b.update_item_route_disabled);
    }

    #[test]
    fn test_disable_schema_routes() {
        let b = CrudRouterBuilder::new::<TestServer>()
            .repository::<Repo>()
            .schema::<Schema, PrimaryKeyType>()
            .disable_list_items_route()
            .disable_get_item_route()
            .disable_delete_item_route()
            .disable_delete_all_items_route();
        assert!(b.list_items_route_disabled);
        assert!(b.get_item_route_disabled);
        assert!(b.delete_item_route_disabled);
        assert!(b.delete_all_items_route_disabled);
        assert!(!b.create_item_route_disabled);
        assert!(!b.update_item_route_disabled);
    }

    #[test]
    fn test_disable_create_schema_route() {
        let b = CrudRouterBuilder::new::<TestServer>()
            .repository::<Repo>()
            .schema::<Schema, PrimaryKeyType>()
            .create_schema::<CreateSchema>()
            .disable_create_item_route();
        assert!(!b.list_items_route_disabled);
        assert!(!b.get_item_route_disabled);
        assert!(!b.delete_item_route_disabled);
        assert!(!b.delete_all_items_route_disabled);
        assert!(b.create_item_route_disabled);
        assert!(!b.update_item_route_disabled);
    }

    #[test]
    fn test_disable_update_schema_route() {
        let b = CrudRouterBuilder::new::<TestServer>()
            .repository::<Repo>()
            .schema::<Schema, PrimaryKeyType>()
            .update_schema::<UpdateSchema>()
            .disable_update_item_route();
        assert!(!b.list_items_route_disabled);
        assert!(!b.get_item_route_disabled);
        assert!(!b.delete_item_route_disabled);
        assert!(!b.delete_all_items_route_disabled);
        assert!(!b.create_item_route_disabled);
        assert!(b.update_item_route_disabled);
    }

    #[test]
    fn test_prefix() {
        let b = CrudRouterBuilder::new::<TestServer>()
            .repository::<Repo>()
            .schema::<Schema, PrimaryKeyType>();

        assert_eq!(b.get_prefix(), "test_table_name");

        let b = b.prefix("test_prefix");
        assert_eq!(b.get_prefix(), "test_prefix");
    }

}
