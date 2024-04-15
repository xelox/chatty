use std::collections::HashMap;
use futures_locks::RwLock;
use axum::extract::ws::WebSocket;

use crate::structs::client::Client;

pub struct ServerState {
    clients_map: RwLock<HashMap<String, Client>>,
}

impl ServerState {
    pub fn new() -> ServerState {
        return ServerState {
            clients_map: RwLock::new(HashMap::new()),
        };
    }

    pub async fn add_client(&self, socket: WebSocket, unique_name: String) {
        let mut clients_map = self.clients_map.write().await;
        clients_map.insert(unique_name.clone(), Client::new(unique_name, socket));
    }

    pub async fn get_client(&self, unique_name: &String) -> Option<Client> {
        let clients_map = self.clients_map.read().await;
        return clients_map.get(unique_name).cloned();
    }
}
