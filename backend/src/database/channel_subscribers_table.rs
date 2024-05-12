
use std::time::SystemTime;

use diesel::deserialize::Queryable;
use serde::Serialize;

use crate::database::{self, schema};
use crate::structs::id::ChattyId;

#[derive(Queryable)]
#[derive(Serialize)]
#[diesel(table_name = schema::channel_subscribers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChannelSubscribers {
    user_id: ChattyId,
    channel_id: ChattyId,
    subscribed_at: SystemTime
}

impl ChannelSubscribers {
    pub fn sorded_subscribed_channels(user_id: &ChattyId) -> Option<Vec<ChattyId>> {
        use schema::channel_subscribers;
        use diesel::prelude::*;

        let conn = &mut database::establish_connection();
        let query: Result<Vec<ChattyId>, _> = channel_subscribers::table
            .filter(channel_subscribers::user_id.eq(user_id))
            .select(channel_subscribers::channel_id)
            .order(channel_subscribers::channel_id.desc())
            .get_results(conn);

        if let Ok(rows) = query {
            Some(rows)
        } else {
            None
        }
    }
}
