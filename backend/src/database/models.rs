use diesel::associations::Identifiable;
use diesel::JoinOnDsl;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::BoolExpressionMethods;
use diesel::SelectableHelper;
use diesel::deserialize::Queryable;
use diesel::prelude::Insertable;
use diesel::Selectable;
use diesel::expression::ValidGrouping;
use serde::Serialize;
use uuid::{NoContext, Uuid};
use crate::database;
use crate::structs::checked_string::CheckedString;
use crate::database::schema;

#[derive(Queryable, Selectable, ValidGrouping)]
#[derive(Clone, Debug)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize)]
pub struct User {
    pub unique_name: CheckedString,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub display_name: Option<String>,
}

impl User {
    /// Needed only for sending the use information about themselves.
    /// TODO: could cache this result on the session store and reduce querys count?
    pub fn query_user(target: &CheckedString) -> Option<User> {
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
    pub unique_name: &'a str,
    pub password_hash: &'a str,
    pub display_name: Option<String>,
}

#[derive(Queryable, Selectable, Insertable, Identifiable)]
#[derive(Serialize)]
#[derive(Clone, Debug)]
#[diesel(table_name = schema::friendship)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Friendship {
    id: Uuid,
    a: String,
    b: String,
    sender: String,
    accepted: bool,
}

pub struct FriendshipTargets {
    a: String,
    b: String,
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct RelationAndUser {
    pub relation: Friendship,
    pub user: User,
}

impl FriendshipTargets {
    pub fn new(target_a: &CheckedString, target_b: &CheckedString) -> FriendshipTargets {
        if target_a > target_b {
            return FriendshipTargets { a: target_b.to_string(), b: target_a.to_string() };
        }
        return FriendshipTargets {a: target_a.to_string(), b: target_b.to_string()};
    }

    fn unpack(&self) -> (String, String) {
        (self.a.clone(), self.b.clone())
    }
} 

impl Friendship {
    pub fn accept(id_: Uuid, acceptor: String) -> bool {
        use schema::friendship::dsl::*;
        let conn = &mut database::establish_connection();
        let query: Result<Friendship, _> = friendship
            .find(id_)
            .select(Friendship::as_select())
            .first(conn);

        if let Ok(r) = query {
            if r.accepted == false && acceptor != r.sender {
                let query = diesel::update(&r)
                    .set((sender.eq(acceptor), accepted.eq(true)))
                    .execute(conn);

                return query.is_ok();
            }
        } 
        return false;
    }

    pub fn create(targets: FriendshipTargets, sender_: &CheckedString) -> Option<Uuid> {
        let (a_, b_) = targets.unpack();
        let ts = uuid::Timestamp::now(NoContext);
        let id_ = Uuid::new_v7(ts);
        let r = Friendship {
            id: id_,
            a: a_,
            b: b_,
            sender: sender_.to_string(),
            accepted: false
        };

        use schema::friendship::dsl::*;
        let conn = &mut database::establish_connection();
        let query = diesel::insert_into(friendship)
            .values(&r)
            .execute(conn);

        if query.is_ok() { Some(id_) }
        else { None }
    }

    pub fn query_user_relations(target: &CheckedString) -> Option<Vec<RelationAndUser>> {
    use schema::friendship;
    use schema::users;
    let conn = &mut database::establish_connection();
    let query: Result<Vec<(User, Friendship)>, diesel::result::Error> = friendship::table
        .inner_join( users::table.on(
            users::unique_name
                .eq(friendship::b)
                .or(users::unique_name.eq(friendship::a))
                .and(users::unique_name.ne(target))
        ))
        .select((users::all_columns, friendship::all_columns))
        .filter(friendship::a.eq(&target)).or_filter(friendship::b.eq(&target))
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
}

// gooffy ah ah this will fail github actions.
#[test]
fn query_user_relations() {
    let target = CheckedString::new(String::from("test")).unwrap();
    assert!(Friendship::query_user_relations(&target).is_some());
}
