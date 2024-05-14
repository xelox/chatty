use diesel::prelude::Insertable;
use diesel::deserialize::Queryable;
use diesel::Selectable;
use serde::Serialize;

use crate::database::{self, schema};
use crate::structs::id::ChattyId;
use crate::structs::ts::TimeStamp;

#[derive(Queryable, Selectable, Insertable)]
#[derive(Serialize)]
#[derive(Clone, Debug)]
#[diesel(table_name = schema::channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChannelTable {
    id: ChattyId,
    channel_name: String,
    channel_description: Option<String>,
    created_at: TimeStamp,
    last_activity: TimeStamp,
    subscribers_count: i32,
}

impl ChannelTable {
    pub fn list_members_ids(channel_id: ChattyId) -> Option<Vec<ChattyId>> {
        use diesel::prelude::*;
        use schema::users;
        use schema::channel_subscribers;

        let conn = &mut database::establish_connection();
        let query: Result<Vec<ChattyId>, _> = channel_subscribers::table.inner_join(users::table.on(
            users::id.eq(channel_subscribers::user_id)
        ))
            .select(users::id)
            .filter(channel_subscribers::channel_id.eq(channel_id))
            .load(conn);

        match query {
            Ok(users) => Some(users),
            Err(_) => None
        }
    }
}
