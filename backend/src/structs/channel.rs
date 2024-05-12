use futures_locks::RwLock;
use super::{client::Client, id::ChattyId};
use crate::database::channel_table::ChannelTable;

pub struct Channel {
    info: ChannelTable,
    id: ChattyId,
    clients: RwLock<Vec<Client>>,
}

impl Channel {
}
