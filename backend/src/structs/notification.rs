use std::{fs::OpenOptions, io::{self, Seek, Write}, rc::Rc, time::{SystemTime, UNIX_EPOCH}};
use uuid::{NoContext, Timestamp, Uuid};

pub struct Notification {
    uuid: Uuid,
    timestamp: u128,
    actions: Rc<[String]>,
    content: String,
    target_unique_name: String,
}

impl Notification {
    pub fn new(target_unique_name: String, content: String) -> Notification {
        return Notification::new_with_actions(target_unique_name, content, Rc::new([]));
    }

    pub fn new_with_actions(target_unique_name: String, content: String, actions: Rc<[String]>) -> Notification {
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

    pub fn new_friend_req(target_unique_name: &String, sender_unique_name: &String, req_id: Uuid) -> Notification {
        let content = format!("New friend request from {}", sender_unique_name);
        return Notification::new(target_unique_name.to_string(), content);
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
