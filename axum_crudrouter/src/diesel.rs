use crate::CRUDGenerator;
use axum::extract::{Path, State};
use axum::{Json, Router};
use diesel::associations::HasTable;
use diesel::connection::LoadConnection;
use diesel::expression::ValidGrouping;
use diesel::helper_types::{delete, AsSelect, Find, Limit, Select, SqlTypeOf, Update};
use diesel::internal::table_macro::{FromClause, SelectStatement, StaticQueryFragment};
use diesel::prelude::*;
use diesel::query_builder::{AsQuery, InsertStatement, IntoUpdateTarget, QueryFragment, QueryId};
use diesel::query_dsl::filter_dsl::FindDsl;
use diesel::query_dsl::methods::{ExecuteDsl, LimitDsl};
use diesel::query_dsl::select_dsl::SelectDsl;
use diesel::query_dsl::LoadQuery;
use diesel::sql_types::SingleValue;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::marker::PhantomData;
use std::sync::Arc;
use axum::routing::get;
use tokio::sync::Mutex;


struct DieselCRUDRouter<DBConnection, SchemaTable, Schema, CreateSchema, UpdateSchema>
where
    DBConnection: Connection,
    SchemaTable: Table + Copy
{
    connection: DBConnection,
    table: SchemaTable,
    _marker: PhantomData<(Schema, CreateSchema, UpdateSchema)>
}

impl<DBConnection, SchemaTable, Schema, CreateSchema, UpdateSchema> DieselCRUDRouter<DBConnection, SchemaTable, Schema, CreateSchema, UpdateSchema>
where
    DBConnection: Connection,
    SchemaTable: Table + Copy
{

    pub fn build(connection: DBConnection, app: Router, table: SchemaTable) -> Router {
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
}


impl<DBConnection, SchemaTable, Schema, CreateSchema, UpdateSchema> CRUDGenerator<Schema, SchemaTable::PrimaryKey> for DieselCRUDRouter<DBConnection, SchemaTable, Schema, CreateSchema, UpdateSchema>
where
    DBConnection: Connection + LoadConnection,
    SchemaTable: AsQuery<Query=SelectStatement<FromClause<SchemaTable>>> + QueryFragment<DBConnection::Backend> + StaticQueryFragment + Table + QueryId + Copy,

    // for list_items_route
    SchemaTable: SelectDsl<AsSelect<Schema, DBConnection::Backend>>,
    Schema: SelectableHelper<DBConnection::Backend>,
    Schema::SelectExpression: QueryId,
    Select<SchemaTable, AsSelect<Schema, DBConnection::Backend>>: Table + Expression,
    for<'a> Select<SchemaTable, AsSelect<Schema, DBConnection::Backend>>: LoadQuery<'a, DBConnection, Schema>,

    // for get_item_route
    SchemaTable: LimitDsl + FindDsl<SchemaTable::PrimaryKey>,
    Find<SchemaTable, SchemaTable::PrimaryKey>: LimitDsl + Table,
    for<'a> Limit<Find<SchemaTable, SchemaTable::PrimaryKey>>: LoadQuery<'a, DBConnection, Schema>,

    // for create_item_route
    CreateSchema: DeserializeOwned + Insertable<SchemaTable>,
    for<'a> InsertStatement<SchemaTable, CreateSchema::Values>: AsQuery + LoadQuery<'a, DBConnection, Schema>,

    // for update_item_route
    UpdateSchema: DeserializeOwned + AsChangeset<Target=SchemaTable>,
    Find<SchemaTable, SchemaTable::PrimaryKey>: HasTable<Table=SchemaTable> + IntoUpdateTarget,
    for<'a> Update<Find<SchemaTable, SchemaTable::PrimaryKey>, UpdateSchema>: AsQuery + LoadQuery<'a, DBConnection, Schema>,

    // for delete_item_route
    SchemaTable::PrimaryKey: SelectableExpression<SchemaTable> + ValidGrouping<()>,
    SqlTypeOf<SchemaTable::PrimaryKey>: SingleValue,
    delete<Find<SchemaTable, SchemaTable::PrimaryKey>>: ExecuteDsl<DBConnection>,

    // for delete_all_items_route
    SchemaTable: IntoUpdateTarget,
    delete<SchemaTable>: ExecuteDsl<DBConnection>
{
    async fn list_items_route(state: State<Arc<Mutex<Self>>>) -> Json<Vec<Schema>> {
        let mut state = state.lock().await;

            state.table.select(Schema::as_select())
            .load::<Schema>(&mut state.connection)
            .expect("Error loading items")
            .into()
    }

    async fn get_item_route(state: State<Arc<Mutex<Self>>>, Path(id): Path<SchemaTable::PrimaryKey>) -> Json<Option<Schema>> {
        let mut state = state.lock().await;

        state.table
            .find(id)
            .first(&mut state.connection)
            .optional()
            .unwrap()
            .into()
    }

    async fn create_item_route(state: State<Arc<Mutex<Self>>>, Json(new_item_json): Json<Value>) -> Json<Schema> {
        let mut state = state.lock().await;

        let new_item = <CreateSchema>::deserialize(new_item_json).unwrap();

        diesel::insert_into(state.table)
            .values(new_item)
            .get_result(&mut state.connection)
            .expect("Updating the post")
            .into()
    }

    async fn update_item_route(state: State<Arc<Mutex<Self>>>, Path(id): Path<SchemaTable::PrimaryKey>, Json(item_json): Json<Value>) -> Json<Schema> {
        let mut state = state.lock().await;

        let item = <UpdateSchema>::deserialize(item_json).unwrap();

        diesel::update(state.table.find(id))
            .set(item)
            .get_result(&mut state.connection)
            .expect("Updating the post")
            .into()
    }

    async fn delete_item_route(state: State<Arc<Mutex<Self>>>, Path(id): Path<SchemaTable::PrimaryKey>) {
        let mut state = state.lock().await;

        diesel::delete(state.table.find(id))
            .execute(&mut state.connection)
            .expect("Error deleting item");
    }

    async fn delete_all_items_route(state: State<Arc<Mutex<Self>>>) -> Json<usize> {
        let mut state = state.lock().await;

        diesel::delete(state.table)
            .execute(&mut state.connection)
            .expect("Error deleting items")
            .into()
    }
}
