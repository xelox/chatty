use std::collections::HashMap;

use futures::lock::Mutex;

use axum::extract::ws::WebSocket;

pub struct ServerState {
    client_sockets: Mutex<HashMap<String, WebSocket>>,
}

impl ServerState {
    pub fn new() -> ServerState {
        return ServerState {
            client_sockets: Mutex::new(HashMap::new()),
        };
    }

    pub async fn add_client(&self, socket: WebSocket, client_id: String) {
        let mut sockets = self.client_sockets.lock().await;
        sockets.insert(client_id, socket);
    }
}
