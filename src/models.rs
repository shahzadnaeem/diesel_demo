use diesel::prelude::*;
use diesel::sql_types::BigInt;
use serde::Serialize;

use crate::schema::{categories, posts};

#[derive(Queryable, Serialize, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable, Serialize, Debug)]
pub struct Category {
    pub id: i32,
    pub value: String,
    pub display_value: String,
    pub display_order: i32,
    pub enum_id: i32,
    pub parent_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = categories)]
pub struct NewCategory<'a> {
    pub value: &'a str,
    pub display_value: &'a str,
    pub display_order: i32,
    pub enum_id: i32,
    pub parent_id: Option<i32>,
}

// NOTE: Model used for direct query of SEQUENCE used to get unique
//       Parent Category Enum ID - see category.rs

#[derive(QueryableByName, Serialize, Debug)]
pub struct Catenumid {
    #[diesel(sql_type = BigInt)]
    pub enum_id: i64,
}
