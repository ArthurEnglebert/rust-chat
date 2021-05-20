#[derive(Queryable)]
pub struct Message {
    pub id: i32,
    pub body: String,
}

use super::schema::posts;

#[derive(Insertable)]
#[table_name="messages"]
pub struct NewMessage<'a> {
    pub title: &'a str,
    pub body: &'a str,
}