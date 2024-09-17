use axum::extract::{Path, State};
use axum::{Json, Router};
use diesel::associations::HasTable;
use diesel::connection::LoadConnection;
use diesel::helper_types::{delete, Find, Limit, Update};
use diesel::internal::table_macro::{FromClause, SelectStatement, StaticQueryFragment};
use diesel::prelude::*;
use diesel::query_builder::{AsQuery, InsertStatement, IntoUpdateTarget, QueryFragment, QueryId};
use diesel::query_dsl::filter_dsl::FindDsl;
use diesel::query_dsl::methods::{ExecuteDsl, LimitDsl};
use diesel::query_dsl::LoadQuery;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use axum::routing::get;
use serde::Serialize;


pub struct DieselCRUDRouter<DBConnection, SchemaTable, Schema, PrimaryKeyType, CreateSchema, UpdateSchema>
{
    connection: DBConnection,
    table: SchemaTable,
    _marker: PhantomData<(Schema, PrimaryKeyType, CreateSchema, UpdateSchema)>
}

impl<DBConnection, SchemaTable, Schema, PrimaryKeyType, CreateSchema, UpdateSchema> DieselCRUDRouter<DBConnection, SchemaTable, Schema, PrimaryKeyType, CreateSchema, UpdateSchema>
where
    DBConnection: Connection + LoadConnection + 'static,
    SchemaTable: AsQuery<Query=SelectStatement<FromClause<SchemaTable>>> + QueryFragment<DBConnection::Backend> + StaticQueryFragment + Table + QueryId + Copy + Send + 'static,

    PrimaryKeyType: Send + DeserializeOwned + 'static,
    SchemaTable::PrimaryKey: EqAll<PrimaryKeyType>,

    // for list_items_route
    Schema: Serialize + Send + 'static,
    for<'a> SchemaTable: LoadQuery<'a, DBConnection, Schema>,

    // for get_item_route
    SchemaTable: LimitDsl + FindDsl<PrimaryKeyType>,
    Find<SchemaTable, PrimaryKeyType>: LimitDsl,
    for<'a> Limit<Find<SchemaTable, PrimaryKeyType>>: LoadQuery<'a, DBConnection, Schema>,

    // for create_item_route
    CreateSchema: DeserializeOwned + Insertable<SchemaTable> + Send + 'static,
    for<'a> InsertStatement<SchemaTable, CreateSchema::Values>: AsQuery + LoadQuery<'a, DBConnection, Schema>,

    // for update_item_route
    UpdateSchema: DeserializeOwned + AsChangeset<Target=SchemaTable> + Send + 'static,
    Find<SchemaTable, PrimaryKeyType>: HasTable<Table=SchemaTable> + IntoUpdateTarget,
    for<'a> Update<Find<SchemaTable, PrimaryKeyType>, UpdateSchema>: AsQuery + LoadQuery<'a, DBConnection, Schema>,

    // for delete_item_route
    delete<Find<SchemaTable, PrimaryKeyType>>: ExecuteDsl<DBConnection>,

    // for delete_all_items_route
    SchemaTable: IntoUpdateTarget,
    delete<SchemaTable>: ExecuteDsl<DBConnection>
{
    pub fn build(connection: DBConnection, table: SchemaTable) -> Router {
        let shared_state = Arc::new(Mutex::new(Self {
            connection,
            table,
            _marker: PhantomData
        }));

        Router::new()
            .route("/", get(Self::list_items_route).post(Self::create_item_route).delete(Self::delete_all_items_route))
            .route("/:id", get(Self::get_item_route).put(Self::update_item_route).delete(Self::delete_item_route))
            .with_state(shared_state)
    }

    async fn list_items_route(state: State<Arc<Mutex<Self>>>) -> Json<Vec<Schema>> {
        let mut state = state.lock().unwrap();

            state.table
            .load::<Schema>(&mut state.connection)
            .expect("Error loading items")
            .into()
    }

    async fn get_item_route(state: State<Arc<Mutex<Self>>>, Path(id): Path<PrimaryKeyType>) -> Json<Option<Schema>> {
        let mut state = state.lock().unwrap();

        state.table.find(id)
            .limit(1)
            .get_result::<Schema>(&mut state.connection)
            .optional()
            .unwrap()
            .into()
    }

    async fn create_item_route(state: State<Arc<Mutex<Self>>>, Json(new_item_json): Json<Value>) -> Json<Schema> {
        let mut state = state.lock().unwrap();

        let new_item = <CreateSchema>::deserialize(new_item_json).unwrap();

        diesel::insert_into(state.table)
            .values(new_item)
            .get_result(&mut state.connection)
            .expect("Updating the post")
            .into()
    }

    async fn update_item_route(state: State<Arc<Mutex<Self>>>, Path(id): Path<PrimaryKeyType>, Json(item_json): Json<Value>) -> Json<Schema> {
        let mut state = state.lock().unwrap();

        let item = <UpdateSchema>::deserialize(item_json).unwrap();

        diesel::update(state.table.find(id))
            .set(item)
            .get_result(&mut state.connection)
            .expect("Updating the post")
            .into()
    }

    async fn delete_item_route(state: State<Arc<Mutex<Self>>>, Path(id): Path<PrimaryKeyType>) {
        let mut state = state.lock().unwrap();

        diesel::delete(state.table.find(id))
            .execute(&mut state.connection)
            .expect("Error deleting item");
    }

    async fn delete_all_items_route(state: State<Arc<Mutex<Self>>>) -> Json<usize> {
        let mut state = state.lock().unwrap();

        diesel::delete(state.table)
            .execute(&mut state.connection)
            .expect("Error deleting items")
            .into()
    }
}
