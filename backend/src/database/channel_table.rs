use std::time::SystemTime;

use diesel::prelude::Insertable;
use diesel::deserialize::Queryable;
use diesel::Selectable;
use serde::Serialize;

use crate::database::schema;
use crate::structs::id::ChattyId;

// id -> ChattyId,
// #[max_length = 255]
// channel_name -> Varchar,
// #[max_length = 255]
// channel_description -> Nullable<Varchar>,
// created_at -> Timestamp,
// last_activity -> Timestamp,
// subscribers_count -> Int4,
//
// The complete User-Relation struct.
#[derive(Queryable, Selectable, Insertable)]
#[derive(Serialize)]
#[derive(Clone, Debug)]
#[diesel(table_name = schema::channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChannelTable {
    id: ChattyId,
    channel_name: String,
    channel_description: Option<String>,
    created_at: SystemTime,
    last_activity: SystemTime,
    subscribers_count: i32,
}
