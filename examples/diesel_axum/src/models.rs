use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::posts;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}


#[derive(Deserialize, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
}


#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = posts)]
pub struct PostForm {
    title: Option<String>,
    body: Option<String>,
    published: Option<bool>,
}