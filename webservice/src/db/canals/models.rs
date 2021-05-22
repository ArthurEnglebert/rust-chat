use crate::db::schema::canals;

#[derive(Queryable)]
pub struct Canal {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="canals"]
pub struct NewCanal<'a> {
    pub name: &'a str,
}

#[derive(AsChangeset)]
#[table_name="canals"]
pub struct UpdateCanal<'a> {
    pub name: Option<&'a str>,
}