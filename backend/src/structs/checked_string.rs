use std::fmt::Display;
use diesel::{deserialize::{FromSql, FromSqlRow}, expression::AsExpression, pg::Pg, serialize::ToSql};
use serde::{de::{Error, Unexpected, Visitor}, Deserialize, Serialize};
use regex::Regex;
use diesel::sql_types::*;


#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = VarChar)]
pub struct CheckedString {
    str: String,
}

#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(FromSqlRow)]
pub struct Email {
    str: String,
}

impl CheckedString {
    fn new(input: String) -> Option<CheckedString> {
        if input.len() > 254 {
            return None;
        }
        if Regex::new(r"^[\w\-\.]+$").unwrap().is_match(&input) {
            return Some(CheckedString { str: input });
        } else {
            return None;
        }
    }
    fn format() -> String {
        String::from("only alpha numerics, underscore, dash or dots.")
    }
}

impl Email {
    fn new(input: String) -> Option<Email> {
        if input.len() > 254 {
            return None;
        }
        if Regex::new(r"^[\w\-\.]+@([\w-]+\.)+[\w-]{2,}$").unwrap().is_match(&input) {
            return Some(Email { str: input });
        } else {
            return None;
        }
    }

    fn format() -> String {
        String::from("an email looks like: email@example.com")
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
impl<'de> Deserialize<'de> for T {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
where
        D: serde::Deserializer<'de> {
        struct TVisitor;
        impl<'de> Visitor<'de> for TVisitor {
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

        deserializer.deserialize_string(TVisitor)
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

#[test]
fn test(){
    let should_work = CheckedString::new(String::from("xelox")); 
    assert_ne!(should_work, None);

    let should_work = CheckedString::new(String::from("xelox_1234")); 
    assert_ne!(should_work, None);

    let should_fail = CheckedString::new(String::from("xelox@&")); 
    assert_eq!(should_fail, None);

    let should_work = Email::new(String::from("example_1234@mail.co.uk")); 
    assert_ne!(should_work, None);

    let should_work = Email::new(String::from("example@mail.co.uk")); 
    assert_ne!(should_work, None);

    let should_fail = CheckedString::new(String::from("bad*example@aaa.bbb")); 
    assert_eq!(should_fail, None);

    let should_fail = CheckedString::new(String::from("bad@example@aaa.bbb")); 
    assert_eq!(should_fail, None);

    let should_fail = CheckedString::new(String::from("bad_example@aaaaaaa")); 
    assert_eq!(should_fail, None);
}
