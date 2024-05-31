use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use futures_locks::RwLock;
use axum::extract::ws::WebSocket;

use crate::database::message_table::{Message, MessageOperations};
use crate::database::users_table::User;
use crate::structs::channel::Channel;
use crate::structs::chatty_response::ChattyResponse;
use crate::structs::client::Client;
use crate::structs::id::ChattyId;
use crate::structs::socket_signal::Signal;

pub struct ServerState {
    clients_map: RwLock<HashMap<ChattyId, Client>>,
    channels_map: RwLock<HashMap<ChattyId, Channel>>
}

impl ServerState {
    pub fn new() -> ServerState {
        return ServerState {
            clients_map: RwLock::new(HashMap::new()),
            channels_map: RwLock::new(HashMap::new())
        };
    }

    pub async fn add_client(&self, socket: WebSocket, id: &ChattyId) {
        let Some(client) = Client::init(id, socket) else {
            return;
        };
        {
            let mut clients_map = self.clients_map.write().await;
            clients_map.insert(*id, client.clone());
        }
        let user_subscribed_channels = client.list_subscribed_channels().await;
        let mut channels_map = self.channels_map.write().await;
        for id in user_subscribed_channels {
            match channels_map.get_mut(&id) {
                Some(channel) => {
                    channel.connect(client.clone());
                }
                None => { 
                    channels_map.insert(id, Channel::load(id, self).await); 
                }
            }
        }
    }

    pub async fn get_client(&self, id: &ChattyId) -> Option<Client> {
        let clients_map = self.clients_map.read().await;
        return clients_map.get(&id).cloned();
    }

    pub async fn get_clients_subset(&self, ids: Vec<ChattyId>) -> HashMap<ChattyId, Client> {
        let mut map = HashMap::new();
        let clients_map = self.clients_map.read().await;

        for id in ids {
            let result = clients_map.get(&id);    
            if let Some(client) = result {
                map.insert(id, client.clone());
            }
        }
        map
    }

    pub async fn broadcast_message(&self, message: Message, op: MessageOperations) -> ChattyResponse {
        let channels_map = self.channels_map.read().await;
        let Some(channel) = channels_map.get(&message.channel_id) else {
            return ChattyResponse::InternalError;
        };
        channel.broadcast_message(message, op).await;
        ChattyResponse::Ok
    }

    pub async fn broadcast_profile_patch(&self, user: User) -> ChattyResponse {
        let client_map = self.clients_map.read().await;
        let channel_map = self.channels_map.read().await;

        let Some(client) = client_map.get(&user.id) else {
            return ChattyResponse::InternalError;
        };
        let order = Arc::new([Signal::ProfilePatch(user)]);
        let channel_ids = client.list_subscribed_channels().await;

        let mut visited_clients = HashSet::<ChattyId>::new();
        let mut futures = Vec::new();

        for channel_id in channel_ids {
            if let Some(channel) = channel_map.get(&channel_id) {
                for (client_id, client) in channel.get_clients() {
                    if visited_clients.get(client_id) == None {
                        visited_clients.insert(*client_id);
                        futures.push(client.send_socket_order(order.clone()))
                    } 
                }
            }
        }

        futures::future::join_all(futures).await;

        ChattyResponse::Ok
    }
}
