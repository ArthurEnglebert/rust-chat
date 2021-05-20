#[derive(Queryable)]
pub struct Message {
    pub id: i32,
    pub body: String,
}