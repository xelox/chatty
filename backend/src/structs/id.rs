use std::{fmt::Display, sync::OnceLock, time::{SystemTime, UNIX_EPOCH}};

use futures_locks::RwLock;
use rand::RngCore;

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct ChattyId {
    id: u64,
}

impl ChattyId {
    pub async fn gen() -> Option<ChattyId> {
        const CHATTY_EPOCH: u64 = 1704067200; // 2024-01-01 00:00
        static RNG: OnceLock<RwLock<rand::rngs::OsRng>> = OnceLock::new();
        let rng = RNG.get_or_init(|| RwLock::new(rand::rngs::OsRng));

        let Ok(ts) = SystemTime::now().duration_since(UNIX_EPOCH) else {
            return None;
        };

        let ts_part = ts.as_secs() - CHATTY_EPOCH;

        let random_part = rng.write().await.next_u32() as u64;

        Some(ChattyId {
            id: ts_part << 32 | random_part
        })
    }
}

impl Display for ChattyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}



#[tokio::test]
async fn id_generation() {
    use tokio::spawn;
    let mut futures = Vec::new();
    for _ in 0..50 {
        futures.push(spawn(ChattyId::gen()));
    }
    for handle in futures {
        let id = handle.await.expect("thread panic!?");
        assert_ne!(id, None);
    }
}
