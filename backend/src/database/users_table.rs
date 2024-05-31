use std::path::Path;

use axum::body::Bytes;
use diesel::prelude::Insertable;
use diesel::deserialize::Queryable;
use diesel::Selectable;
use diesel::expression::ValidGrouping;
use serde::Serialize;
use crate::database;
use crate::file_storage::save;
use crate::structs::chatty_response::ChattyResponse;
use crate::structs::checked_string::CheckedString;
use crate::database::schema;
use crate::structs::checked_string::Email;
use crate::structs::id::ChattyId;
use crate::structs::ts::TimeStamp;



#[derive(Queryable, Selectable, ValidGrouping)]
#[derive(Clone, Debug)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize)]
pub struct User {
    pub id: ChattyId,

    pub username: CheckedString,
    pub email: Option<Email>,

    #[serde(skip_serializing)]
    pub password_hash: String,

    pub display_name: Option<String>,
    pub has_pfp: bool,
    pub has_banner: bool,
    pub custom_status: Option<String>,
    pub about_me: String,

    pub created_at: TimeStamp,
    pub last_online: TimeStamp,
}

impl User {
    /// Needed only for sending the use information about themselves.
    /// TODO: could cache this result on the session store and reduce querys count?
    pub fn query_user(id: &ChattyId) -> Option<User> {
        use diesel::prelude::*;
        use schema::users;
        let conn = &mut database::establish_connection();
        let query: Result<User, _> = users::table
            .find(id)
            .first(conn);

        if let Ok(user) = query {
            Some(user)        
        } else {
            None
        }
    }

    pub fn query_channels(&self) -> Option<Vec<ChattyId>> {
        use diesel::prelude::*;
        use schema::users;
        use schema::channel_subscribers;
        let conn = &mut database::establish_connection();
        let query: Result<Vec<ChattyId>, _> = users::table.inner_join(
            channel_subscribers::table.on(
                channel_subscribers::user_id.eq(users::id)
            ))
            .filter(users::id.eq(self.id))
            .select(channel_subscribers::channel_id)
            .load(conn);

        match query {
            Ok(ids) => Some(ids),
            Err(_) => None
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
        
    pub fn update_profile_decorations(decorations: Vec<ProfileDecoration>, uid: ChattyId) -> Option<User> {
        use schema::users;
        use diesel::prelude::*;


        #[derive(AsChangeset)]
        #[diesel(table_name = schema::users)]
        struct Change<'a> {
            display_name: Option<&'a str>,
            has_pfp: Option<&'a bool>,
            has_banner: Option<&'a bool>,
            about_me: Option<&'a str>,
            custom_status: Option<&'a str>,
        }

        let mut changes = Change {
            display_name: None,
            has_pfp: None,
            has_banner: None,
            about_me: None,
            custom_status: None
        };

        for decoration in decorations.iter() {
            match decoration {
                ProfileDecoration::Pfp(bytes) => {
                    let path_str = format!("pfp/{uid}.png");
                    let _ = save(Path::new(&path_str), &bytes);
                    changes.has_pfp = Some(&true);
                },
                ProfileDecoration::Banner(bytes) => {
                    let path_str = format!("banner/{uid}.png");
                    let _ = save(Path::new(&path_str), &bytes);
                    changes.has_banner = Some(&true);
                },
                ProfileDecoration::DisplayName(display_name) => {
                    changes.display_name = Some(display_name);
                }
                ProfileDecoration::AboutMe(about_me) => {
                    changes.about_me = Some(about_me);
                },
                ProfileDecoration::Status(status) => {
                    changes.custom_status = Some(status);
                }
            }
        }

        let conn = &mut database::establish_connection();
        let query: Result<User, _> = diesel::update(users::table.filter(users::id.eq(uid)))
            .set(changes)
            .returning(users::all_columns)
            .get_result(conn);

        match query {
            Ok(user) => Some(user),
            Err(_) => None,
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

pub enum ProfileDecoration {
    DisplayName(String),
    Pfp(Bytes),
    Banner(Bytes),
    AboutMe(String),
    Status(String),
}
