use std::sync::Arc;
use axum::extract::ws::WebSocket;
use futures_locks::{Mutex, RwLock};
use crate::structs::id::ChattyId;

use crate::database::users_table::User;

use super::socket_signal::{Signal, SignalList};

#[derive(Clone)]
pub struct Client {
    socket: Mutex<WebSocket>,
    user_model: Arc<User>,
    channels_subscribed_to: RwLock<Vec<ChattyId>>,
    status: RwLock<String>, //TODO: status class
}

impl Client {
    pub fn init(id: &ChattyId, socket: WebSocket) -> Option<Client> {
        let query = User::query_user(id);
        let Some(user) = query else {
            return None;
        };

        let query = user.query_channels();
        let Some(channels) = query else {
            return None;
        };

        return Some(Client {
            socket: Mutex::new(socket),
            user_model: Arc::new(user),
            channels_subscribed_to: RwLock::new(channels),
            status: RwLock::new(String::from("online")),
        });
    }

    pub async fn send_socket_order(&self, signals: Arc<[Signal]>) {
        let mut socket = self.socket.lock().await;
        if let Some(message) = SignalList::new(signals).to_message() {
            let _ = socket.send(message).await;
        }
    }

    pub fn get_id(&self) -> ChattyId {
        self.user_model.id
    }

    pub async fn list_subscribed_channels(&self) -> Vec<ChattyId> {
        self.channels_subscribed_to.read().await.to_vec()
    }
}


