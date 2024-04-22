use diesel::{expression::ValidGrouping, prelude::*};
use uuid::{NoContext, Uuid};
use crate::{database::{self, schema}, structs::checked_string::CheckedString};

#[derive(Queryable, Selectable, ValidGrouping)]
#[derive(Clone, Debug)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub unique_name: CheckedString,
    pub password_hash: String,
    pub display_name: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser<'a> {
    pub unique_name: &'a str,
    pub password_hash: &'a str,
    pub display_name: Option<String>,
}

#[derive(Queryable, Selectable, Insertable, Identifiable)]
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
}
