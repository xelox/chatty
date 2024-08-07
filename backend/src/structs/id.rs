use core::panic;
use std::{env, fmt::Display, sync::OnceLock, time::{SystemTime, UNIX_EPOCH}, u32};

use diesel::{deserialize::{FromSql, FromSqlRow}, expression::AsExpression, pg::Pg, serialize::ToSql, sql_types::BigInt};
use futures_locks::Mutex;
use serde::{de::{Error, Unexpected, Visitor}, Deserialize, Serialize};

const CHATTY_EPOCH: u64 = 1704067200; // 2024-01-01 00:00
type IdType = u64;

const TS_BITS: u32 = 34;
const NODE_ID_BITS: u32 = 10;
const SEQUENCE_BITS: u32 = 20;

const MAX_TS: u64 = 2u64.pow(TS_BITS) - 1;
const MAX_NODE_ID: u16 = 2u16.pow(NODE_ID_BITS) - 1;
const MAX_SEQUENCE: u32 = 2u32.pow(SEQUENCE_BITS) - 1;

#[derive(Hash)]
#[derive(Clone, Debug, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = BigInt)]
pub struct ChattyId {
    id: IdType,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PackedId {
    pub ts: u64,
    pub node_id: u16,
    pub seq: u32,
}

impl PackedId {
    fn unpack(&self) -> Option<ChattyId> {
        if self.ts > MAX_TS { return None; }
        if self.node_id > MAX_NODE_ID { return None; }
        if self.seq > MAX_SEQUENCE { return None; }

        let id: u64 = (self.ts << (NODE_ID_BITS + SEQUENCE_BITS)) 
        | ((self.node_id as u64) << SEQUENCE_BITS) 
        | self.seq as u64;

        Some(ChattyId { id })
    }
}

struct Sequencer {
    clock: u64,
    seq: u32,
}

impl Sequencer {
    fn new() -> Sequencer {
        let Ok(ts) = SystemTime::now().duration_since(UNIX_EPOCH) else {
            println!("It looks like time seriously went backwords.");
            std::process::exit(-1);
        };

        Sequencer{clock: ts.as_secs() - CHATTY_EPOCH, seq: 0 }
    }
    
    fn next(&mut self) -> ChattyId {
        static NODE_ID: OnceLock::<u16> = OnceLock::new();
        let node_id = *NODE_ID.get_or_init(|| {
            let Ok(node_id) = env::var("CHATTY_NODE_ID") else {
                println!("Chatty Node ID needs to be set.");
                std::process::exit(-2);
            };
            let Ok(node_id) = node_id.parse::<u16>() else {
                println!("Chatty Node ID needs to be a u16");
                std::process::exit(-2);
            };
            if node_id > MAX_NODE_ID {
                println!("Chatty Node ID needs to be smaller or equeal to {MAX_NODE_ID}");
                std::process::exit(-2);
            }
            node_id
        });
        
        let Ok(ts) = SystemTime::now().duration_since(UNIX_EPOCH) else {
            println!("It looks like time seriously went backwords.");
            std::process::exit(-1);
        };

        let ts = ts.as_secs() - CHATTY_EPOCH;
        

        self.seq += 1;

        if self.seq > MAX_SEQUENCE {
            self.clock += 1;
            self.seq = 0;
        }

        if ts > self.clock {
            self.clock = ts;
            self.seq = 0;
        }

        if self.clock > MAX_TS {
            println!("It has been too long a time, it's time to move on.");
            std::process::exit(-69);
        }

        let packed = PackedId {
            ts: self.clock, node_id, seq: self.seq
        };

        packed.unpack().unwrap_or_else(||{
            println!("There is a bug in the sequencer.");
            std::process::exit(-2);
        })
    }
}

impl ChattyId {
    pub async fn gen() -> ChattyId {
        static SEQUENCER: OnceLock<Mutex<Sequencer>> = OnceLock::new();
        let sequencer = SEQUENCER.get_or_init(|| Mutex::new(Sequencer::new()));

        sequencer.lock().await.next()
    }

    pub fn pack(&self) -> PackedId {
        const NODE_ID_MASK: u64 = !(u64::MAX << (NODE_ID_BITS + SEQUENCE_BITS));
        const SEQ_ID_MASK: u64 = !(u64::MAX << SEQUENCE_BITS);

        let ts = (self.id >> (NODE_ID_BITS + SEQUENCE_BITS )) as u64;
        let node_id = ((self.id & NODE_ID_MASK) >> SEQUENCE_BITS ) as u16;
        let seq = (self.id & SEQ_ID_MASK) as u32;

        PackedId {ts, node_id, seq}
    }
}

impl Default for ChattyId {
    fn default() -> Self {
        Self{id: 0}
    }
}

impl Display for ChattyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}


impl FromSql<BigInt, Pg> for ChattyId {
    fn from_sql(bytes: <Pg as diesel::backend::Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        <i64 as FromSql<BigInt, Pg>>::from_sql(bytes).map(|s| Self{id: s as IdType})
    }
}

impl ToSql<BigInt, Pg> for ChattyId {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        <i64 as ToSql<diesel::sql_types::BigInt, Pg>>::to_sql(&(self.id as i64), &mut out.reborrow())
    }
}

impl Serialize for ChattyId
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(self.id.to_string().as_str())
    }
}


impl<'de> Deserialize<'de> for ChattyId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
where
        D: serde::Deserializer<'de> {
        struct TVisitor;
        impl<'de> Visitor<'de> for TVisitor {
            type Value = ChattyId;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a number")    
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
            E: serde::de::Error, {
                match v.parse::<u64>() {
                    Ok(id) => Ok(ChattyId{id}),
                    Err(_) => Err(Error::invalid_value(Unexpected::Str(&v.to_string()), &self))
                }
            }
        }

        deserializer.deserialize_str(TVisitor)
    }
}

#[tokio::test]
async fn id_generation() {
    env::set_var("CHATTY_NODE_ID", "7");

    use tokio::spawn;
    let mut futures = Vec::new();

    for _ in 0..10 {
        futures.push(spawn(ChattyId::gen()));
    }
    for handle in futures {
        let _ = handle.await.unwrap();
    }
}

#[test] 
fn pack_unpack() {
    env::set_var("CHATTY_NODE_ID", "7");

    let fail_1 = PackedId {
        ts: u64::MAX,
        node_id: 1,
        seq: 1,
    };
    assert_eq!(fail_1.unpack(), None);

    let fail_2 = PackedId {
        ts: 1,
        node_id: u16::MAX,
        seq: 1,
    };
    assert_eq!(fail_2.unpack(), None);
    
    let fail_3 = PackedId {
        ts: 1,
        node_id: 1,
        seq: u32::MAX,
    };
    assert_eq!(fail_3.unpack(), None);

    let pass_1 = PackedId {
        ts: 1,
        node_id: 1,
        seq: 1,
    };

    assert_ne!(pass_1.unpack(), None);
    let pass_1_id = pass_1.unpack().unwrap();
    assert_eq!(pass_1_id.pack(), pass_1);
}

#[test]
fn bits_usage() {
    let internal_type_size = std::mem::size_of::<IdType>() * 8;
    assert_eq!(internal_type_size, (TS_BITS + NODE_ID_BITS + SEQUENCE_BITS) as usize);
}

#[tokio::test]
async fn serialize_test() {
    env::set_var("CHATTY_NODE_ID", "7");

    let id = ChattyId::gen().await;
    let serialized_str = serde_json::to_string(&id);
    assert!(serialized_str.is_ok(), "Serializing fails.");
    let deserialized_id = serde_json::from_str::<ChattyId>(&serialized_str.unwrap());
    assert!(deserialized_id.is_ok(), "Deserializing fails.");
    assert_eq!(deserialized_id.unwrap(), id, "ID -> JSON -> ID fails equality check.");

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct Test {
        name: String,
        id: ChattyId,
    }

    let real_usage_test_instance = Test { name: "John Snow".to_string(), id };
    let real_usage_test_str = format!("{{\"name\":\"John Snow\",\"id\":\"{id}\"}}");

    let serr: Test = serde_json::from_str(&real_usage_test_str).unwrap();
    let deserr = serde_json::to_string(&real_usage_test_instance).unwrap();

    assert_eq!(serr, real_usage_test_instance);
    assert_eq!(deserr, real_usage_test_str);
}
