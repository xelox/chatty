use std::{collections::HashMap, usize};
use diesel::{deserialize::{FromSqlRow, Queryable}, expression::AsExpression, prelude::Insertable, sql_types::Jsonb};
use crate::{database::{self, schema}, structs::{chatty_response::ChattyResponse, ts::TimeStamp}};
use serde::{Deserialize, Serialize};
use crate::structs::id::ChattyId;

// id -> Int8,
// sender_id -> Int8,
// channel_id -> Int8,
// #[max_length = 2000]
// content -> Varchar,
// attachments -> Array<Nullable<Text>>,
// mentions -> Array<Nullable<Int8>>,
// reactions -> Nullable<Jsonb>,
// sent_at -> Timestamp,
// updated_at -> Timestamp,

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Queryable)]
#[diesel(table_name = schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: ChattyId,
    pub sender_id: ChattyId,
    pub channel_id: ChattyId,
    pub content: String,
    pub sent_at: TimeStamp,
    pub updated_at: Option<TimeStamp>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMessage {
    #[serde(skip_deserializing)]
    pub id: ChattyId,
    pub sender_id: ChattyId,
    pub channel_id: ChattyId,
    pub content: String
}

impl Message {
    /// WARNING dosen't ensure that the sender is authorized!
    pub async fn store(message: &mut NewMessage) -> Option<Message> {
        use schema::messages;
        use diesel::prelude::*;

        let id = ChattyId::gen().await;
        message.id = id;
        
        let conn = &mut database::establish_connection();
        let query: Result<Message, diesel::result::Error> = diesel::insert_into(messages::table)
            .values(&*message)
            .get_result(conn);

        match query {
            Ok(message) => Some(message),
            Err(_) => None
        }
    }
}
