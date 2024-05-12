use std::collections::HashMap;
use futures_locks::RwLock;
use axum::extract::ws::WebSocket;

use crate::structs::client::Client;
use crate::structs::id::ChattyId;

pub struct ServerState {
    clients_map: RwLock<HashMap<ChattyId, Client>>,
}

impl ServerState {
    pub fn new() -> ServerState {
        return ServerState {
            clients_map: RwLock::new(HashMap::new()),
        };
    }

    pub async fn add_client(&self, socket: WebSocket, id: &ChattyId) {
        let mut clients_map = self.clients_map.write().await;
        let Some(client) = Client::init(id, socket) else {
            return;
        };
        clients_map.insert(*id, client);
    }

    pub async fn get_client(&self, id: &ChattyId) -> Option<Client> {
        let clients_map = self.clients_map.read().await;
        return clients_map.get(&id).cloned();
    }
}
