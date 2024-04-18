use std::sync::Arc;

use axum::extract::ws::Message;
use serde::Serialize;

use super::notification::Notification;

#[derive(Serialize)]
pub struct MessageItem {
    sender: String,
    channel: uuid::Uuid,
    timestamp: u64,
    id: uuid::Uuid,
}

#[derive(Serialize)]
pub struct FriendReqItem {
    sender: String,
    receiver: String,
    timestamp: u64,
    id: uuid::Uuid,
}

#[derive(Serialize)]
pub struct FriendListItem {
    target: String,
    status: String,
    friend: bool,
}

#[derive(Serialize)]
pub enum Signal {
    Message(MessageItem),
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

    pub fn to_message(&self) -> Option<Message> {
        if let Ok(json) = serde_json::to_string(self) {
            Some(Message::Text(json))
        } else {
            None
        }
    }
}
