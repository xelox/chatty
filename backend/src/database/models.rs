use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[derive(Clone, Debug)]
#[diesel(table_name = crate::database::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub unique_name: String,
    pub password_hash: String,
    pub display_name: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::users)]
pub struct NewUser<'a> {
    pub unique_name: &'a str,
    pub password_hash: &'a str,
    pub display_name: Option<String>,
}
