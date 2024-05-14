use std::collections::HashMap;

use super::{client::Client, id::ChattyId};
use crate::{database::channel_table::ChannelTable, server_state::ServerState};

pub struct Channel {
    clients: HashMap<ChattyId, Client>,
}

impl Channel {
    async fn load(id: ChattyId, state: ServerState) -> Option<Channel> {
        let Some(members) = ChannelTable::list_members_ids(id) else {
            return None;
        };
        let clients = state.get_clients_subset(members).await;
        Some(Channel { clients })
    }

    pub fn connect(&mut self, client: Client) {
        self.clients.insert(client.get_id(), client);
    }

    pub fn disconnect(&mut self, client_id: &ChattyId) {
        self.clients.remove(client_id);
    }
}
