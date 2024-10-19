use diesel::associations::HasTable;
use diesel::connection::LoadConnection;
use diesel::helper_types::{delete, Find, Limit, Offset, Update};
use diesel::internal::table_macro::{FromClause, SelectStatement, StaticQueryFragment};
use diesel::prelude::*;
use diesel::query_builder::{AsQuery, InsertStatement, IntoUpdateTarget, QueryFragment, QueryId};
use diesel::query_dsl::filter_dsl::FindDsl;
use diesel::query_dsl::methods::{ExecuteDsl, LimitDsl, OffsetDsl};
use diesel::query_dsl::LoadQuery;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::Pagination;
use crate::repositories::{CRUDRepository, CreateRepository, ReadDeleteRepository, UpdateRepository};

pub struct DieselRepository<DBConnection, SchemaTable> {
    connection: DBConnection,
    table: SchemaTable
}


impl<DBConnection, SchemaTable> DieselRepository<DBConnection, SchemaTable>
where
    SchemaTable: Table,
{
    pub fn new(connection: DBConnection, table: SchemaTable) -> Self{
        Self{
            connection,
            table
        }
    }

}

impl<DBConnection, SchemaTable> CRUDRepository for DieselRepository<DBConnection, SchemaTable> {}

impl<DBConnection, SchemaTable, Schema, PrimaryKeyType> ReadDeleteRepository<Schema, PrimaryKeyType> for DieselRepository<DBConnection, SchemaTable>
where
    DBConnection: Connection + LoadConnection + 'static,
    SchemaTable: AsQuery<Query=SelectStatement<FromClause<SchemaTable>>> + QueryFragment<DBConnection::Backend> + StaticQueryFragment<Component=diesel::internal::table_macro::Identifier<'static>> + Table + QueryId + Copy + Send + 'static,

    PrimaryKeyType: Send + DeserializeOwned + 'static,
    SchemaTable::PrimaryKey: EqAll<PrimaryKeyType>,

    // for list_items
    Schema: Serialize + Send + 'static,
    for<'a> Offset<Limit<SelectStatement<FromClause<SchemaTable>>>>: LoadQuery<'a, DBConnection, Schema>,
    for<'a> Limit<SelectStatement<FromClause<SchemaTable>>>: LoadQuery<'a, DBConnection, Schema>,
    for<'a> Offset<SelectStatement<FromClause<SchemaTable>>>: LoadQuery<'a, DBConnection, Schema>,
    for<'a> SchemaTable: LoadQuery<'a, DBConnection, Schema>,

    // for get_item
    SchemaTable: LimitDsl + FindDsl<PrimaryKeyType>,
    Find<SchemaTable, PrimaryKeyType>: LimitDsl,
    for<'a> Limit<Find<SchemaTable, PrimaryKeyType>>: LoadQuery<'a, DBConnection, Schema>,

    // for delete_item
    Find<SchemaTable, PrimaryKeyType>: HasTable<Table=SchemaTable> + IntoUpdateTarget,
    delete<Find<SchemaTable, PrimaryKeyType>>: ExecuteDsl<DBConnection>,

    // for delete_all_items
    SchemaTable: IntoUpdateTarget,
    delete<SchemaTable>: ExecuteDsl<DBConnection>

{
    fn get_table_name() -> String {
        <SchemaTable as StaticQueryFragment>::STATIC_COMPONENT.0.to_string()
    }

    async fn list_items(&mut self, pagination: Pagination) -> Vec<Schema> {
        let result = match (pagination.limit, pagination.skip) {
            (Some(limit), Some(skip)) =>
                OffsetDsl::offset(
                    LimitDsl::limit(self.table.as_query(), limit as i64),
                    skip as i64
                )
                .load::<Schema>(&mut self.connection),
            (Some(limit), None) =>
                LimitDsl::limit(self.table.as_query(), limit as i64)
                .load::<Schema>(&mut self.connection),
            (None, Some(skip)) =>
                OffsetDsl::offset(self.table.as_query(), skip as i64)
                .load::<Schema>(&mut self.connection),
            (None, None) =>
                self.table.load::<Schema>(&mut self.connection),
        };
        result.expect("Error loading items")
    }

    async fn get_item(&mut self, id: PrimaryKeyType) -> Option<Schema> {
        self.table
            .find(id)
            .limit(1)
            .get_result::<Schema>(&mut self.connection)
            .optional()
            .unwrap()
    }
    async fn delete_item(&mut self, id: PrimaryKeyType) {
        diesel::delete(self.table.find(id))
            .execute(&mut self.connection)
            .expect("Error deleting item");
    }

    async fn delete_all_items(&mut self) -> usize {
        diesel::delete(self.table)
            .execute(&mut self.connection)
            .expect("Error deleting items")
    }
}

impl<DBConnection, SchemaTable, Schema, CreateSchema> CreateRepository<Schema, CreateSchema> for DieselRepository<DBConnection, SchemaTable>
where
    DBConnection: Connection + LoadConnection + 'static,
    SchemaTable: AsQuery<Query=SelectStatement<FromClause<SchemaTable>>> + QueryFragment<DBConnection::Backend> + StaticQueryFragment + Table + QueryId + Copy + Send + 'static,

    // for create_item
    CreateSchema: DeserializeOwned + Insertable<SchemaTable> + Send + 'static,
    for<'a> InsertStatement<SchemaTable, CreateSchema::Values>: AsQuery + LoadQuery<'a, DBConnection, Schema>,
{
    async fn create_item(&mut self, new_item: CreateSchema) -> Schema {
        diesel::insert_into(self.table)
            .values(new_item)
            .get_result(&mut self.connection)
            .expect("Error creating item")
            .into()
    }
}

impl<DBConnection, SchemaTable, Schema, PrimaryKeyType, UpdateSchema> UpdateRepository<Schema, PrimaryKeyType, UpdateSchema> for DieselRepository<DBConnection, SchemaTable>
where
    DBConnection: Connection + LoadConnection + 'static,
    SchemaTable: AsQuery<Query=SelectStatement<FromClause<SchemaTable>>> + QueryFragment<DBConnection::Backend> + StaticQueryFragment + Table + QueryId + Copy + Send + 'static,

    PrimaryKeyType: Send + DeserializeOwned + 'static,
    SchemaTable::PrimaryKey: EqAll<PrimaryKeyType>,

    // for get_item
    SchemaTable: LimitDsl + FindDsl<PrimaryKeyType>,
    Find<SchemaTable, PrimaryKeyType>: LimitDsl,
    for<'a> Limit<Find<SchemaTable, PrimaryKeyType>>: LoadQuery<'a, DBConnection, Schema>,

    // for update_item
    UpdateSchema: DeserializeOwned + AsChangeset<Target=SchemaTable> + Send + 'static,
    Find<SchemaTable, PrimaryKeyType>: HasTable<Table=SchemaTable> + IntoUpdateTarget,
    for<'a> Update<Find<SchemaTable, PrimaryKeyType>, UpdateSchema>: AsQuery + LoadQuery<'a, DBConnection, Schema>,
{
    async fn update_item(&mut self, id: PrimaryKeyType, item: UpdateSchema) -> Schema {
        diesel::update(self.table.find(id))
            .set(item)
            .get_result(&mut self.connection)
            .expect("Error updating item")
    }
}