use std::{fs::OpenOptions, io::{self, Seek, Write}, time::{SystemTime, UNIX_EPOCH}};
use uuid::Uuid;

pub struct Notification {
    uuid: Uuid,
    timestamp: u64,
    content: String,
    target_unique_name: String,
}

impl Notification {
    pub fn new(target_unique_name: String, content: String) -> Notification {
        return Notification {
            uuid: Uuid::new_v4(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            content,
            target_unique_name
        };
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
