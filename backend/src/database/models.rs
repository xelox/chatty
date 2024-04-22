use diesel::{expression::ValidGrouping, prelude::*};
use serde::Serialize;
use uuid::{NoContext, Uuid};
use crate::{database::{self, schema::{self}}, structs::checked_string::CheckedString};

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

    pub fn query_user_relations(target: &CheckedString) -> Option<Vec<((String, Option<String>), Friendship)>> {
    use schema::friendship;
    use schema::users;
    let conn = &mut database::establish_connection();
    let query: Result<Vec<((String, Option<String>), Friendship)>, diesel::result::Error> = friendship::table
        .inner_join( users::table.on(
            users::unique_name
                .eq(friendship::b)
                .or(users::unique_name.eq(friendship::a))
                .and(users::unique_name.ne(friendship::sender))
        ))
        .select(((users::unique_name, users::display_name), friendship::all_columns))
        .filter(friendship::a.eq(&target)).or_filter(friendship::b.eq(&target))
        .load(conn);

        if let Ok(result) = query {
            Some(result)
        } else {
            None
        }
    }
}

#[test]
fn name() {
    let target = String::from("pablo");

    use schema::friendship;
    use schema::users;
    let conn = &mut database::establish_connection();

    let query: Result<Vec<((String, Option<String>), Friendship)>, diesel::result::Error> = friendship::table
        .inner_join( users::table.on(
            users::unique_name
                .eq(friendship::b)
                .or(users::unique_name.eq(friendship::a))
                .and(users::unique_name.ne(friendship::sender))
        ))
        .select(((users::unique_name, users::display_name), friendship::all_columns))
        .filter(friendship::a.eq(&target)).or_filter(friendship::b.eq(&target))
        .load(conn);

    let Ok(rows) = query else {
        panic!("Query error");
    };

    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut buf = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    let Ok(_) = rows.serialize(&mut ser) else {
        panic!("Faild to pase to json");
    };

    let json = String::from_utf8(buf).unwrap();
    
    println!("{json}");


    panic!();
}
