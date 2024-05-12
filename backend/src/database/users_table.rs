use std::time::SystemTime;
use diesel::prelude::Insertable;
use diesel::deserialize::Queryable;
use diesel::Selectable;
use diesel::expression::ValidGrouping;
use serde::Serialize;
use crate::database;
use crate::structs::checked_string::CheckedString;
use crate::database::schema;
use crate::structs::checked_string::Email;
use crate::structs::id::ChattyId;



#[derive(Queryable, Selectable, ValidGrouping)]
#[derive(Clone, Debug)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize)]
pub struct User {
    pub id: ChattyId,
    pub username: CheckedString,
    pub email: Option<Email>,
    pub display_name: Option<String>,

    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: SystemTime,
    pub last_online: SystemTime,
}

impl User {
    /// Needed only for sending the use information about themselves.
    /// TODO: could cache this result on the session store and reduce querys count?
    pub fn query_user(target: &ChattyId) -> Option<User> {
        use diesel::prelude::*;
        use schema::users;
        let conn = &mut database::establish_connection();
        let query: Result<User, _> = users::table
            .find(target)
            .first(conn);

        if let Ok(user) = query {
            Some(user)        
        } else {
            None
        }
    }

    pub fn query_user_by_username(target: &CheckedString) -> Option<User> {
        use diesel::prelude::*;
        use schema::users;
        let conn = &mut database::establish_connection();
        let query: Result<User, _> = users::table
            .filter(users::username.eq(target))
            .first(conn);

        if let Ok(user) = query {
            Some(user)        
        } else {
            None
        }
    }

    pub async fn create_user(unique_name: &CheckedString, password: &String) -> Option<ChattyId> {
        use diesel::prelude::*;
        use schema::users;
        let conn = &mut database::establish_connection();
        let password_hash = password_auth::generate_hash(&password);
        let id = ChattyId::gen().await;

        let res: Result<User, _> = diesel::insert_into(schema::users::table)
            .values(NewUser{
                id: &id,
                username: &unique_name.to_string(),
                display_name: None,
                password_hash: &password_hash,
            })
            .returning(users::all_columns)
            .get_result(conn);

        if res.is_ok() {
            return Some(id);
        } else {
            return None;
        }
    }

    pub fn validate_password(unique_name: &CheckedString, password: &String) -> AuthValidationResult {
        use schema::users;
        use diesel::prelude::*;
        let conn = &mut database::establish_connection();

        let user_search: Result<User, _> = users::table
            .filter(users::username.eq(unique_name))
            .first(conn);

        match user_search {
            Ok(user) => {
                match password_auth::verify_password(&password, &user.password_hash) {
                    Ok(()) => {
                        AuthValidationResult::Valid(user.id)
                    }
                    _ => {
                        AuthValidationResult::IncorrectPassword
                    }
                }
            }
            _ => {
                AuthValidationResult::IncorrectUniqueName
            }
        }
    }

}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
struct NewUser<'a> {
    pub id: &'a ChattyId, 
    pub username: &'a str,
    pub password_hash: &'a str,
    pub display_name: Option<&'a String>,
}

pub enum AuthValidationResult {
    Valid(ChattyId),
    IncorrectUniqueName,
    IncorrectPassword,
}
