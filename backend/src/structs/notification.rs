use std::{fs::OpenOptions, io::{self, Seek, Write}, sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use serde::Serialize;
use uuid::{NoContext, Timestamp, Uuid};
use crate::config::HOSTNAME;

use super::checked_string::CheckedString;

#[derive(Serialize, Clone)]
pub struct Notification {
    id: Uuid,
    kind: String,
    timestamp: u128,
    actions: Arc<[String]>,
    content: String,
    target: Uuid,
}

impl Notification {
    pub fn new_friend_req(target: &Uuid, sender_unique_name: &Uuid, req_id: &Uuid) -> Notification {
        let id = Uuid::new_v7(Timestamp::now(NoContext));
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
