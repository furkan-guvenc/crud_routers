# crud_routers

**crud_routers** is a library to automatically generate crud routes with given schemas.
It is orm and web framework agnostic, highly inspired by [fastapi-crudrouter](https://github.com/awtkns/fastapi-crudrouter).

[![Crates.io](https://img.shields.io/crates/v/crud_routers)](https://crates.io/crates/crud_routers)
[![Documentation](https://docs.rs/crud_routers/badge.svg)](https://docs.rs/crud_routers)

## Basic usage with Axum and Diesel

### Installation
```bash
cargo add crud_routers --features axum,diesel
```

### Usage
Below is a simple example of what the crud_routers can do. In just ten lines of code, you can generate all
the crud_routers you need for any model. The full example is in [diesel_example folder](examples/diesel_axum)

```rust
#[tokio::main]
async fn main() -> io::Result<()> {
    let database_url = "postgres://postgres:testpw@localhost/diesel_demo";
    let connection = PgConnection::establish(&database_url).unwrap();
    let shared_state = Arc::new(Mutex::new(
        DieselRepository::new(connection, posts::table)
    ));

    let router = CrudRouterBuilder::new::<AxumServer>()
        .schema::<Post, i32>()
        .create_schema::<NewPost>()
        .update_schema::<PostForm>()
        .prefix("base/api")
        .build_router()
        .with_state(shared_state);

    axum::serve(listener, router).await
}
```

## Features

### Orm-Agnostic

Following ORMs are implemented, and you can activate them with adding necessary features.

- [Diesel](https://diesel.rs/) with feature "diesel"
- [Sea-orm](https://www.sea-ql.org/SeaORM/) with feature "sea-orm"

You can easily add new ones by implementing [necessary traits](crud_routers/src/repositories/mod.rs).

### Api Server Agnostic
Following api servers are implemented, and you can activate them with adding necessary features.
You can mix and match them with Orms however you want. 

- [Axum](https://github.com/tokio-rs/axum) with feature "axum"
- [Actix](https://actix.rs/) with feature "actix"

### OpenApi support
You can easily add [openapi](https://www.openapis.org/) support with feature "openapi" and 
deriving [utoipa::ToSchema](https://docs.rs/utoipa/latest/utoipa/derive.ToSchema.html) for your schemas.
You can use all UIs supported by [utoipa](https://github.com/juhaku/utoipa).

```rust
let mut openapi = OpenApiBuilder::new()
.info(InfoBuilder::new().title("Diesel Axum example").build())
.build();

let router = CrudRouterBuilder::new::<AxumServer>()
        .schema::<Post, i32>()
        .create_schema::<NewPost>()
        .update_schema::<PostForm>()
        .prefix("base/api")
        .build_openapi(&mut openapi)
        .build_router()
        .with_state(shared_state)
        .merge(SwaggerUi::new("/docs/swagger/").url("/api-docs/openapi.json", openapi));

```

![Swagger UI](docs/assets/SwaggerOverview.png)

### Pagination
Pagination is automatically paginate setup for you. You can use the `skip` and `limit` query parameters to
paginate your results.

**Skip**:
Using the `skip` (int) parameter, you can skip a certain number of items before returning the items you want.

**Limit**:
Using the `limit` (int) parameter, the maximum number of items to be returned can be defined.

![Swagger UI](docs/assets/ListAll.png)

### Opting Out Routes
If you don't add a schema with `create_schema` then create item route won't be created.
Same applies for `update_schema` method and update item route.
Alternatively all routes can be opted out using disable_*_route methods.

```rust
CrudRouterBuilder::new::<TestServer>()
.repository::<Repo>()
.schema::<Schema, PrimaryKeyType>()
.create_schema::<CreateSchema>()
.update_schema::<UpdateSchema>()
.disable_list_items_route()
.disable_get_item_route()
.disable_delete_item_route()
.disable_delete_all_items_route()
.disable_create_item_route()
.disable_update_item_route()
```

### Set tag and prefix
You can set a prefix for your url with `prefix` method.
Leaving prefix makes it the table name.
If "openapi" feature is added then `tag` method 
can be used to set the tag for the api spec.

```rust
CrudRouterBuilder::new::<TestServer>()
.repository::<Repo>()
.schema::<Schema, PrimaryKeyType>()
.prefix("base/api")
.tag("My Tag")
```

### TODO

- [ ] Add Middleware support
- [ ] Create an [mdBook](https://github.com/rust-lang/mdBook) for documentation
