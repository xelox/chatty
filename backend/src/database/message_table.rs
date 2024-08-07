use diesel::{deserialize::Queryable, prelude::Insertable};
use crate::{database::{self, schema}, structs::ts::TimeStamp};
use serde::{Deserialize, Serialize};
use crate::structs::id::ChattyId;

const MESSAGE_LOAD_LIMIT: i64 = 30;

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

#[derive(Deserialize)]
pub struct NewMessage {
    pub sender_id: ChattyId,
    pub channel_id: ChattyId,
    pub content: String
}

#[derive(Deserialize)]
pub struct ExistingMessage {
    pub id: ChattyId,
    pub sender_id: ChattyId,
    pub channel_id: ChattyId,
    pub content: String
}

impl Message {
    /// WARNING dosen't ensure that the sender is authorized!
    pub async fn store(message: &NewMessage) -> Option<Message> {
        use schema::messages;
        use diesel::prelude::*;

        #[derive(Insertable)]
        #[diesel(table_name = schema::messages)]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        struct NewMessageInDB<'a> {
            id: &'a ChattyId,
            sender_id: &'a ChattyId,
            channel_id: &'a ChattyId,
            content: &'a String
        }

        let id = ChattyId::gen().await;

        let save = NewMessageInDB {
            id: &id,
            sender_id: &message.sender_id,
            channel_id: &message.channel_id,
            content: &message.content
        };
        
        let conn = &mut database::establish_connection();
        let query: Result<Message, diesel::result::Error> = diesel::insert_into(messages::table)
            .values(save)
            .get_result(conn);

        match query {
            Ok(message) => Some(message),
            Err(_) => None
        }
    }

    pub fn delete(message: &ExistingMessage) -> Option<Message> {
        use schema::messages;
        use diesel::prelude::*;
        
        let conn = &mut database::establish_connection();
        let query: Result<Message, diesel::result::Error> = diesel::delete(messages::table)
            .filter(messages::id.eq(message.id))
            .returning(messages::all_columns)
            .get_result(conn);
        
        match query {
            Ok(message) => Some(message),
            _ => None
        }
    }

    pub fn load_from_ts(channel_id: &ChattyId, ts: &TimeStamp) -> Option<Vec<Message>> {
        use schema::messages;
        use diesel::prelude::*;

        let conn = &mut database::establish_connection();
        let query: Result<Vec<Message>, _> = messages::table
            .filter(messages::channel_id.eq(channel_id))
            .filter(messages::sent_at.le(ts))
            .limit(MESSAGE_LOAD_LIMIT)
            .order(messages::sent_at.desc())
            .select(messages::all_columns)
            .load(conn);

        match query {
            Ok(messages) => Some(messages),
            _ => None
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub enum MessageOperations {
    Send,
    Delete,
    Patch,
}
