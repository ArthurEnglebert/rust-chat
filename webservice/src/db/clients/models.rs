use crate::db::schema::clients;

#[derive(Queryable)]
pub struct Client {
    pub uuid: String,
    pub name: String,
    pub pass: String,
    pub salt: String,
}

#[derive(Insertable)]
#[table_name="clients"]
pub struct NewClient<'a> {
    pub uuid: &'a str,
    pub name: &'a str,
    pub pass: &'a str,
    pub salt: &'a str,
}

#[derive(AsChangeset)]
#[table_name="clients"]
pub struct UpdateClient<'a> {
    pub name: Option<&'a str>,
    pub pass: Option<&'a str>,
    pub salt: Option<&'a str>,
}