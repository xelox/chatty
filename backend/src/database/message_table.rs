use std::{collections::HashMap, time::SystemTime};
use diesel::{deserialize::Queryable, pg::Pg, prelude::Insertable};
use crate::{database::{self, schema}, structs::chatty_response::ChattyResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


// messages (id) {
//     id -> Uuid,
//     sender_id -> Uuid,
//     channel_id -> Uuid,
//     #[max_length = 2000]
//     content -> Varchar,
//     attachments -> Array<Nullable<Text>>,
//     mentions -> Array<Nullable<Uuid>>,
//     reactions -> Nullable<Jsonb>,
//     sent_at -> Timestamp,
//     updated_at -> Timestamp,
// }

#[derive(Queryable)]
#[derive(Serialize)]
#[diesel(table_name = schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub channel_id: Uuid,
    pub content: String,
    pub attachments: Vec<String>,
    pub mentions: Vec<Uuid>,
    pub reactions: Option<HashMap<String, Vec<Uuid>>>,
    pub sent_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMessage {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub sender_id: Uuid,
    pub channel_id: Uuid,
    pub content: String
}

impl Message {
    /// WARNING dosen't ensure that the sender is authorized!
    pub fn store(message: &mut NewMessage) -> ChattyResponse {
        use schema::messages;
        use diesel::prelude::*;

        let id = Uuid::now_v7();
        message.id = id;
        
        let conn = &mut database::establish_connection();
        let query = diesel::insert_into(messages::table)
            .values(&*message)
            .execute(conn);

        if query.is_ok() {
            ChattyResponse::Ok
        } else {
            let err = query.unwrap_err();
            let query = diesel::insert_into(messages::table)
                .values(&*message);
            let query_str = diesel::debug_query::<Pg, _>(&query).to_string();
            dbg!(err);
            println!("{query_str}");
            ChattyResponse::InternalError
        }
    }
}

// INSERT INTO "messages" ("id", "sender_id", "channel_id", "content") 
// VALUES ('018f2043-e662-7c89-a10c-a5bdd3ee237d', '018f203c-b923-7632-99f9-4f7b5361c073', '018f203c-d149-7529-8489-1b2f0c2318d7', 'test');
