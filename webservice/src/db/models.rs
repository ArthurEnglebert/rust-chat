use super::schema::messages;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Message {
    pub id: i32,
    pub body: String,
}

#[derive(Insertable)]
#[table_name="messages"]
pub struct NewMessage<'a> {
    pub body: &'a str,
}

#[derive(AsChangeset)]
#[table_name="messages"]
pub struct MessageForm<'a> {
    pub body: Option<&'a str>
}