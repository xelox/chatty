use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[derive(Clone, Debug)]
#[diesel(table_name = crate::database::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub nick: String,
    pub password_hash: String,
    pub auth_token: Option<uuid::Uuid>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::users)]
pub struct NewUser<'a> {
    pub nick: &'a str,
    pub password_hash: &'a str,
}
