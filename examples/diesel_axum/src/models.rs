use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use crate::schema::posts;

#[derive(Serialize, Queryable, Selectable, ToSchema)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}


#[derive(Deserialize, Insertable, ToSchema)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
}


#[derive(Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = posts)]
pub struct PostForm {
    title: Option<String>,
    body: Option<String>,
    published: Option<bool>,
}