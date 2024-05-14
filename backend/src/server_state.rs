use std::collections::HashMap;
use futures_locks::RwLock;
use axum::extract::ws::WebSocket;

use crate::structs::channel::Channel;
use crate::structs::client::Client;
use crate::structs::id::ChattyId;

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
}
