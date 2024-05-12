use futures_locks::RwLock;
use uuid::Uuid;
use super::client::Client;
use crate::database::channel_table::ChannelTable;

pub struct Channel {
    info: ChannelTable,
    id: Uuid,
    clients: RwLock<Vec<Client>>,
}

impl Channel {
}
