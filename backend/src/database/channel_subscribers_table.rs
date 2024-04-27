
use std::time::SystemTime;

use diesel::deserialize::Queryable;
use serde::Serialize;
use uuid::Uuid;
use crate::database::{self, schema};

#[derive(Queryable)]
#[derive(Serialize)]
#[diesel(table_name = schema::channel_subscribers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChannelSubscribers {
    user_id: Uuid,
    channel_id: Uuid,
    subscribed_at: SystemTime
}

impl ChannelSubscribers {
    pub fn sorded_subscribed_channels(user_id: &Uuid) -> Option<Vec<Uuid>> {
        use schema::channel_subscribers;
        use diesel::prelude::*;

        let conn = &mut database::establish_connection();
        let query: Result<Vec<Uuid>, _> = channel_subscribers::table
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
