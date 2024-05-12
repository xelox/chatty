use std::{fs::OpenOptions, io::{self, Seek, Write}, sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use serde::Serialize;
use crate::config::HOSTNAME;
use crate::structs::id::ChattyId;

#[derive(Serialize, Clone)]
pub struct Notification {
    id: ChattyId,
    kind: String,
    timestamp: u128,
    actions: Arc<[String]>,
    content: String,
    target: ChattyId,
}

impl Notification {
    pub async fn new_friend_req(target: &ChattyId, sender_unique_name: &ChattyId, req_id: &ChattyId) -> Notification {
        let id = ChattyId::gen().await;
        let kind = String::from("friend_req");
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let actions = Arc::new([format!("{HOSTNAME}/api/accept_friend_request/{req_id}")]);
        let content = format!("New friend request from {}", sender_unique_name);

        return Notification{
            id,
            kind,
            timestamp,
            actions,
            content,
            target: *target,
        };
    }

    pub fn store(&self) -> bool {
        let path = format!("/home/alex/dev/chatty/private/client_notifications/{}", self.target);
        let file_req = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path);

        if let Ok(mut file) = file_req {
            if let Err(_) = file.seek(io::SeekFrom::End(0)) {
                return false;
            }

            let data = format!("{},{},{}\n", self.id, self.timestamp, self.content);
            if let Ok(_) = file.write(data.as_bytes()) {
                return true;
            }
        }
        return false;
    }
}
