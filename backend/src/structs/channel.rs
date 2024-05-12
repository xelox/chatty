use futures_locks::RwLock;
use uuid::Uuid;
use super::client::Client;

pub struct Channel {
    id: Uuid,
    clients: RwLock<Vec<Client>>,
}

impl Channel {

}
