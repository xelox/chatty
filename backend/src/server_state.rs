use std::collections::HashMap;
use futures_locks::RwLock;
use axum::extract::ws::WebSocket;

use crate::structs::client::Client;

pub struct ServerState {
    client_sockets: RwLock<HashMap<String, Client>>,
}

impl ServerState {
    pub fn new() -> ServerState {
        return ServerState {
            client_sockets: RwLock::new(HashMap::new()),
        };
    }

    pub async fn add_client(&self, socket: WebSocket, unique_name: String) {
        let mut sockets = self.client_sockets.write().await;
        sockets.insert(unique_name.clone(), Client::new(unique_name, socket));
    }

    pub async fn get_client(&self, unique_name: &String) -> Option<Client> {
        let sockets = self.client_sockets.read().await;
        return sockets.get(unique_name).cloned();
    }
}
