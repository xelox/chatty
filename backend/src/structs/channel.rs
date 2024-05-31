use std::{collections::HashMap, sync::Arc};

use super::{client::Client, id::ChattyId, socket_signal::Signal};
use crate::{database::{channel_table::ChannelTable, message_table::{Message, MessageOperations}}, server_state::ServerState};

pub struct Channel {
    clients: HashMap<ChattyId, Client>,
}

impl Channel {
    pub async fn load(id: ChattyId, state: &ServerState) -> Channel {
        let members = ChannelTable::list_members_ids(id).unwrap();
        let clients = state.get_clients_subset(members).await;
        Channel { clients }
    }

    pub fn connect(&mut self, client: Client) {
        self.clients.insert(client.get_id(), client);
    }

    pub fn disconnect(&mut self, client_id: &ChattyId) {
        self.clients.remove(client_id);
    }

    pub async fn broadcast_message(&self, message: Message, op: MessageOperations) {
        let order = Arc::new([Signal::Message(op, message)]);
        let mut futures = Vec::new();
        for client in self.clients.values() {
            futures.push(client.send_socket_order(order.clone()));
        }
        futures::future::join_all(futures).await;
    }

    pub fn get_clients(&self) -> &HashMap<ChattyId, Client> {
        &self.clients
    }
}
