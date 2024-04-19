use std::fmt::Display;
use diesel::{deserialize::{FromSql, FromSqlRow}, expression::AsExpression, pg::Pg, serialize::ToSql};
use serde::{de::{Error, Unexpected, Visitor}, Deserialize, Serialize};
use regex::Regex;
use diesel::sql_types::*;


#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Deserialize)]
#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = VarChar)]
pub struct CheckedString {
    str: String,
}

#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(FromSqlRow)]
#[derive(Deserialize)]
pub struct Email {
    str: String,
}

impl CheckedString {
    fn new(input: String) -> Option<CheckedString> {
        if input.len() > 254 {
            return None;
        }
        if Regex::new(r"/^[\w\-\.]+$/gm").unwrap().is_match(&input) {
            return Some(CheckedString {
                str: input
            });
        } else {
            return None;
        }
    }
    fn format() -> String {
        String::from("example-unique_name.1234")
    }
}

impl Email {
    fn new(input: String) -> Option<Email> {
        if input.len() > 254 {
            return None;
        }
        if Regex::new(r"/^[\w\-\.]+@([\w-]+\.)+[\w-]{2,}$/gm").unwrap().is_match(&input) {
            return Some(Email {
                str: input
            });
        } else {
            return None;
        }
    }

    fn format() -> String {
        String::from("email@example.com")
    }
}

use duplicate::duplicate_item;

#[duplicate_item(T; [Email]; [CheckedString])]
impl Serialize for T
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(&self.str)
    }
}

#[duplicate_item(T; [Email]; [CheckedString])]
impl<'de> Visitor<'de> for T {
    type Value = T;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(T::format().as_str())    
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        let input = String::from(&v);
        match T::new(input) {
            Some(t) => {
                Ok(t)
            },
            None => {
                Err(Error::invalid_value(Unexpected::Str(&v), &self))
            }
        }
    }
}

#[duplicate_item(T; [Email]; [CheckedString])]
impl Display for T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str)
    }
}

#[duplicate_item(T; [Email]; [CheckedString])]
impl FromSql<VarChar, Pg> for T {
    fn from_sql(bytes: <Pg as diesel::backend::Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        <String as FromSql<VarChar, Pg>>::from_sql(bytes).map(|s| T{str: s.parse().unwrap()})
    }
}

#[duplicate_item(T; [Email]; [CheckedString])]
impl ToSql<VarChar, Pg> for T {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        <std::string::String as ToSql<diesel::sql_types::VarChar, Pg>>::to_sql(&self.str, &mut out.reborrow())
    }
}
