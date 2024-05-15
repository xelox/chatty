use std::sync::Arc;

use axum::extract::ws::{self};
use serde::Serialize;

use crate::database::message_table::Message;

use super::{id::ChattyId, notification::Notification};

#[derive(Serialize)]
pub struct FriendReqItem {
    sender: String,
    receiver: String,
    timestamp: u64,
    id: ChattyId,
}

#[derive(Serialize)]
pub struct FriendListItem {
    target: String,
    status: String,
    friend: bool,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub enum Signal {
    Message(Message),
    Notification(Notification),
    FriendReq(FriendReqItem),
    FriendListChanged(FriendListItem),
}

#[derive(Serialize)]
pub struct SignalList{
    signals: Arc<[Signal]>
}

impl SignalList {
    pub fn new(signals: Arc<[Signal]>) -> SignalList {
        SignalList {signals}
    }

    pub fn to_message(&self) -> Option<ws::Message> {
        if let Ok(json) = serde_json::to_string(self) {
            Some(ws::Message::Text(json))
        } else {
            None
        }
    }
}
