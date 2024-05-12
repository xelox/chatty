use std::{fmt::Display, sync::OnceLock, time::{SystemTime, UNIX_EPOCH}, u128};

use diesel::{deserialize::{FromSql, FromSqlRow}, expression::AsExpression, pg::Pg, serialize::ToSql, sql_types::BigInt};
use futures_locks::RwLock;
use rand::RngCore;
use serde::{de::{Error, Unexpected, Visitor}, Deserialize, Serialize};

#[derive(Hash)]
#[derive(Clone, Debug, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = BigInt)]
pub struct ChattyId {
    id: u64,
}

impl ChattyId {
    pub async fn gen() -> ChattyId {
        const CHATTY_EPOCH: u64 = 1704067200; // 2024-01-01 00:00
        static RNG: OnceLock<RwLock<rand::rngs::OsRng>> = OnceLock::new();
        let rng = RNG.get_or_init(|| RwLock::new(rand::rngs::OsRng));

        let Ok(ts) = SystemTime::now().duration_since(UNIX_EPOCH) else {
            unreachable!();
        };

        let ts_part = ts.as_secs() - CHATTY_EPOCH;

        let random_part = rng.write().await.next_u32() as u64;

        ChattyId { id: ts_part << 32 | random_part }
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
        <i64 as FromSql<BigInt, Pg>>::from_sql(bytes).map(|s| Self{id: s as u64})
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
        serializer.serialize_u64(*(&self.id))
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
                formatter.write_str("Failed to deserialize chatty id")    
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                E: serde::de::Error, {
                Ok(ChattyId{id: v})
            }

            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
        where
                E: serde::de::Error, {
                match u64::try_from(v) {
                    Ok(id) => Ok(ChattyId{id}),
                    Err(_) => Err(Error::invalid_value(Unexpected::Str(&v.to_string()), &self))
                }
            }
        }

        deserializer.deserialize_u64(TVisitor)
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
        let id = handle.await.unwrap();
    }
}
