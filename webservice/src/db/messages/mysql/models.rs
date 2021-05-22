use crate::db::schema::messages;
use chrono::prelude::*;

#[derive(Queryable)]
pub struct Message {
    pub id: i32,
    pub body: String,
    pub client: String,
    pub date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="messages"]
pub struct NewMessage<'a> {
    pub client: &'a str,
    pub body: &'a str,
    pub date: NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name="messages"]
pub struct UpdateMessage<'a> {
    pub body: Option<&'a str>
}