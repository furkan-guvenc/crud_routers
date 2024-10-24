use std::fs;
use utoipa::openapi::{InfoBuilder, OpenApi, OpenApiBuilder};
use utoipa::{ToSchema};
use crud_routers::{ApiServer, CRUDRepository, CrudRouterBuilder, Pagination, ReadDeleteRepository};

#[derive(ToSchema)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}


#[derive(ToSchema)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
}


#[derive(ToSchema)]
pub struct PostForm {
    title: Option<String>,
    body: Option<String>,
    published: Option<bool>,
}
struct PrimaryKeyType;
struct Repo;
impl CRUDRepository for Repo {}

impl ReadDeleteRepository<Post, PrimaryKeyType> for Repo {
    fn get_table_name() -> String {
        String::from("test_table_name")
    }

    async fn list_items(&mut self, _pagination: Pagination) -> Vec<Post> {
        unimplemented!()
    }

    async fn get_item(&mut self, _id: PrimaryKeyType) -> Option<Post> {
        unimplemented!()
    }

    async fn delete_item(&mut self, _id: PrimaryKeyType) {
        unimplemented!()
    }

    async fn delete_all_items(&mut self) -> usize {
        unimplemented!()
    }
}

struct TestServer;
impl ApiServer for TestServer {
    fn get_id_path(prefix: &str) -> String {
        unimplemented!()
    }
}

fn get_default_openapi() -> OpenApi {
    OpenApiBuilder::new()
        .info(InfoBuilder::new()
            .title("Test api")
            .version("0.1.0")
            .build())
        .build()
}

#[test]
fn openapi_spec() {
    let mut api = get_default_openapi();

    let b = CrudRouterBuilder::new::<TestServer>()
        .repository::<Repo>()
        .prefix("base/api")
        .tag("table_name")
        .schema::<Post, PrimaryKeyType>()
        .create_schema::<NewPost>()
        .update_schema::<PostForm>()
        .build_openapi(&mut api);

    let expected_api_spec = fs::read_to_string("tests/test_api_spec.json").unwrap();

    assert_eq!(api.to_json().unwrap(), expected_api_spec);
}

#[test]
fn openapi_spec_without_create_schema() {
    let mut api = get_default_openapi();

    let b = CrudRouterBuilder::new::<TestServer>()
        .repository::<Repo>()
        .prefix("base/api")
        .tag("table_name")
        .schema::<Post, PrimaryKeyType>()
        .update_schema::<PostForm>()
        .build_openapi(&mut api);

    let expected_api_spec = fs::read_to_string("tests/test_api_spec_without_create_schema.json").unwrap();

    assert_eq!(api.to_json().unwrap(), expected_api_spec);
}
