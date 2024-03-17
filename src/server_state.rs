use futures::{future::join_all, lock::Mutex};

use axum::extract::ws::WebSocket;

pub struct ServerState {
    pub i: Mutex<u32>,
    client_sockets: Mutex<Vec<WebSocket>>,
}

impl ServerState {
    pub fn new() -> ServerState {
        return ServerState {
            i: Mutex::new(0),
            client_sockets: Mutex::new(Vec::new()),
        };
    }

    pub async fn add_client(&self, socket: WebSocket) {
        let mut sockets = self.client_sockets.lock().await;
        sockets.push(socket);
    }

    pub async fn broadcast(&self, msg: String) {
        let mut sockets = self.client_sockets.lock().await;
        let message = axum::extract::ws::Message::Text(msg);
        let mut futures = Vec::new();
        for socket in sockets.iter_mut() {
            futures.push(socket.send(message.clone()));
        }
        join_all(futures).await;
    }
}
