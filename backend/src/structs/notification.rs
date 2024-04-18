use std::{fs::OpenOptions, io::{self, Seek, Write}, sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use serde::Serialize;
use uuid::{NoContext, Timestamp, Uuid};
use crate::config::HOSTNAME;

#[derive(Serialize, Clone)]
pub struct Notification {
    uuid: Uuid,
    timestamp: u128,
    actions: Arc<[String]>,
    content: String,
    target_unique_name: String,
}

impl Notification {
    pub fn new(target_unique_name: String, content: String) -> Notification {
        return Notification::new_with_actions(target_unique_name, content, Arc::new([]));
    }

    pub fn new_with_actions(target_unique_name: String, content: String, actions: Arc<[String]>) -> Notification {
        let ts_base = SystemTime::now().duration_since(UNIX_EPOCH).unwrap(); 
        let timestamp = ts_base.as_millis();
        let seconds = ts_base.as_secs();
        let nanos = (ts_base.as_nanos() - (seconds as u128) * 10u128.pow(9)) as u32;
        let ts = Timestamp::from_unix(NoContext, seconds, nanos);
        return Notification {
            uuid: Uuid::new_v7(ts),
            timestamp,
            actions,
            content,
            target_unique_name
        };
    }

    pub fn new_friend_req(target_unique_name: &String, sender_unique_name: &String, req_id: &Uuid) -> Notification {
        let content = format!("New friend request from {}", sender_unique_name);
        let accept_action = format!("{HOSTNAME}/api/accept_friend_request/{req_id}");
        return Notification::new_with_actions(target_unique_name.to_string(), content, Arc::new([accept_action]));
    }

    pub fn store(&self) -> bool {
        let path = format!("/home/alex/dev/chatty/private/client_notifications/{}", self.target_unique_name);
        let file_req = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path);

        if let Ok(mut file) = file_req {
            if let Err(_) = file.seek(io::SeekFrom::End(0)) {
                return false;
            }

            let data = format!("{},{},{}\n", self.uuid, self.timestamp, self.content);
            if let Ok(_) = file.write(data.as_bytes()) {
                return true;
            }
        }
        return false;
    }
}
