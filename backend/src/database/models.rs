use std::time::SystemTime;

use diesel::associations::Identifiable;
use diesel::debug_query;
use diesel::pg::Pg;
use diesel::query_builder::DebugQuery;
use diesel::JoinOnDsl;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::BoolExpressionMethods;
use diesel::deserialize::Queryable;
use diesel::prelude::Insertable;
use diesel::Selectable;
use diesel::expression::ValidGrouping;
use serde::Deserialize;
use serde::Serialize;
use uuid::{NoContext, Uuid};
use crate::database;
use crate::structs::chatty_response::ChattyResponse;
use crate::structs::checked_string::CheckedString;
use crate::database::schema;
use crate::structs::checked_string::Email;

#[derive(Queryable, Selectable, ValidGrouping)]
#[derive(Clone, Debug)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: CheckedString,
    pub email: Option<Email>,
    pub display_name: String,

    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: SystemTime,
    pub last_online: SystemTime,
}

impl User {
    /// Needed only for sending the use information about themselves.
    /// TODO: could cache this result on the session store and reduce querys count?
    pub fn query_user(target: &Uuid) -> Option<User> {
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
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
    pub display_name: Option<String>,
}

#[derive(Queryable, Selectable, Insertable, Identifiable)]
#[derive(Serialize)]
#[derive(Clone, Debug)]
#[diesel(table_name = schema::user_relations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRelation {
    id: Uuid,
    a: Uuid,
    b: Uuid,
    sender: Uuid,
    accepted: bool,
    created_at: SystemTime,
    accepted_at: Option<SystemTime>,
}


#[derive(Serialize)]
#[derive(Debug)]
pub struct RelationAndUser {
    pub relation: UserRelation,
    pub user: User,
}

pub struct UserRelationPair<'a> {
    a: &'a Uuid,
    b: &'a Uuid,
}

impl<'a> UserRelationPair<'a> {
    pub fn new(target_a: &'a Uuid, target_b: &'a Uuid) -> UserRelationPair<'a> {
        if target_a > target_b {
            return UserRelationPair { a: target_b, b: target_a };
        }
        return UserRelationPair {a: target_a, b: target_b };
    }

    fn unpack(&self) -> (&Uuid, &Uuid) {
        (self.a, self.b)
    }
} 

#[derive(Insertable)]
#[diesel(table_name = schema::user_relations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUserRelation<'a> {
    id: &'a Uuid,
    a: &'a Uuid,
    b: &'a Uuid,
    sender: &'a Uuid,
}

impl UserRelation {
    pub fn create(targets: UserRelationPair, sender_: &Uuid) -> Option<Uuid> {
        let (a_, b_) = targets.unpack();
        let ts = uuid::Timestamp::now(NoContext);
        let id_ = Uuid::new_v7(ts);


        let r = NewUserRelation {
            id: &id_,
            a: a_,
            b: b_,
            sender: sender_
        };

        use schema::user_relations::dsl::*;
        let conn = &mut database::establish_connection();
        let query = diesel::insert_into(user_relations)
            .values(&r)
            .execute(conn);

        if query.is_ok() { Some(id_) }
        else { None }
    }

    pub fn query_user_relations(target: &Uuid) -> Option<Vec<RelationAndUser>> {
        use schema::user_relations;
        use schema::users;
        let conn = &mut database::establish_connection();
        let query: Result<Vec<(User, UserRelation)>, diesel::result::Error> = user_relations::table
            .inner_join(users::table.on(
                users::id.eq(user_relations::b)
                    .or(users::id.eq(user_relations::a))
                    .and(users::id.ne(target))
            ))
            .select((users::all_columns, user_relations::all_columns))
            .filter(user_relations::a.eq(target)).or_filter(user_relations::b.eq(target))
            .load(conn);

        if let Ok(result) = query {
            let mapped_result = result.into_iter().map(|item| {
                let (user, relation) = item;
                RelationAndUser{ relation, user }
            }) .collect();
            Some(mapped_result)
        } else {
            None
        }
    }

    pub fn edit_relation(request_maker: &Uuid, id: Uuid, method: EditFriendshipEnum) -> ChattyResponse {
        use diesel::result::Error;
        use schema::user_relations;
        let conn = &mut database::establish_connection();
        let query: Result<UserRelation, Error> = user_relations::table
            .find(id)
            .select(user_relations::all_columns)
            .first(conn);
        if let Ok(relation) = query {

            match method {
                EditFriendshipEnum::Cancel => {
                    if relation.sender != *request_maker {
                        return  ChattyResponse::Unauthorized;
                    }
                    if relation.accepted {
                        return ChattyResponse::BadRequest(
                            Some("It's too late to cancel this request.".to_string())
                        );
                    }
                },
                EditFriendshipEnum::Remove => {
                    if !relation.accepted {
                        return ChattyResponse::BadRequest(
                            Some("To be removed, a friendship must first be accepted.".to_string())
                        );
                    }
                },
                EditFriendshipEnum::Accept => {
                    if *request_maker == relation.sender {
                        return ChattyResponse::Unauthorized
                    }
                    if relation.accepted {
                        return ChattyResponse::BadRequest(
                            Some("Friendship already accepted.".to_string())
                        );
                    }
                },
                EditFriendshipEnum::Refuse => {
                    if *request_maker == relation.sender {
                        return ChattyResponse::BadRequest(
                            Some("Can't refuse a friendship you sent yourself.".to_string())
                        );
                    }
                    if relation.accepted {
                        return ChattyResponse::BadRequest(
                            Some("Can't refuse a friendship that has already been accepted.".to_string())
                        );
                    }
                }
            }

            let query = user_relations::table.find(id);
            let res: Result<_, Error>;
            if method == EditFriendshipEnum::Accept {
                res = diesel::update(query).set(user_relations::accepted.eq(true)).execute(conn);
            } else {
                res = diesel::delete(query).execute(conn);
            }
            if res.is_err() {
                return ChattyResponse::InternalError;
            }
            return ChattyResponse::Ok;
        } else {
            let error = query.unwrap_err();
            match error {
                Error::NotFound => {
                    return ChattyResponse::BadRequest(None);
                }
                _ => {
                    return ChattyResponse::InternalError;
                }
            }
        };
    }
}

#[derive(PartialEq, Eq)]
#[derive(Deserialize)]
#[serde(rename_all="snake_case")]
pub enum EditFriendshipEnum {
    Cancel,
    Remove,
    Accept,
    Refuse,
}
