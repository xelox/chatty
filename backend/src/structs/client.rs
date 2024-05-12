use std::sync::Arc;
use axum::extract::ws::WebSocket;
use futures_locks::{Mutex, RwLock};
use crate::structs::id::ChattyId;

use crate::database::users_table::User;

use super::socket_signal::{Signal, SignalList};

#[derive(Clone)]
pub struct Client {
    socket: Option<Mutex<WebSocket>>,
    user_model: Arc<User>,
    status: RwLock<String>, //TODO: status class
}

impl Client {
    pub fn init(id: &ChattyId, socket: WebSocket) -> Option<Client> {
        let query = User::query_user(id);
        let Some(user) = query else {
            return None;
        };

        return Some(Client {
            user_model: Arc::new(user),
            socket: Some(Mutex::new(socket)),
            status: RwLock::new(String::from("online")),
        });
    }

    pub async fn send_socket_order(&self, signals: Arc<[Signal]>) {
        if let Some(socket_mutex) = &self.socket {
            let mut socket = socket_mutex.lock().await;
            if let Some(message) = SignalList::new(signals).to_message() {
                let _ = socket.send(message).await;
            }
        }
    }
}


