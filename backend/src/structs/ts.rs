use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::u128;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::ToSql;
use diesel::sql_types::Timestamp;
use diesel::{deserialize::FromSqlRow, expression::AsExpression};
use serde::de::Visitor;
use serde::{Deserialize, Serialize};

use crate::structs::id::ChattyId;


#[derive(Clone, Debug, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = Timestamp)]
pub struct TimeStamp {
    ts: u128,
}

impl TimeStamp {
    fn now() -> TimeStamp {
        TimeStamp {
            ts: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time really did go backwords")
                .as_millis()
        }
    }
}

impl FromSql<Timestamp, Pg> for TimeStamp {
    fn from_sql(bytes: <Pg as diesel::backend::Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        <SystemTime as FromSql<Timestamp, Pg>>::from_sql(bytes).map(|s| {
            let ts = s.duration_since(UNIX_EPOCH)
                .expect("time really did go backwords")
                .as_millis();

            Self{ts}
        })
    }
}

impl ToSql<Timestamp, Pg> for TimeStamp {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let secs = (self.ts / 1000) as u64;
        let millis = (self.ts % 1000) as u32;

        let d = Duration::from_secs(secs).add(Duration::from_millis(millis.into()));
        let t = UNIX_EPOCH.add(d);

        <SystemTime as ToSql<diesel::sql_types::Timestamp, Pg>>::to_sql(&t, &mut out.reborrow())
    }
}

impl Serialize for TimeStamp
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_u128(self.ts)
    }
}


impl<'de> Deserialize<'de> for TimeStamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
        D: serde::Deserializer<'de> {
        struct TVisitor;
        impl<'de> Visitor<'de> for TVisitor {
            type Value = TimeStamp;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Failed to deserialize chatty id")    
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                E: serde::de::Error, {
                Ok(TimeStamp{ts: v as u128})
            }

            fn visit_u128<E>(self, ts: u128) -> Result<Self::Value, E>
                where
                E: serde::de::Error, {
                    Ok(TimeStamp{ts})
                }
        }

        deserializer.deserialize_u64(TVisitor)
    }
}
