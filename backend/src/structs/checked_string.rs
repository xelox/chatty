use std::fmt::Display;
use serde::{de::{Error, Unexpected, Visitor}, Deserialize, Serialize};
use regex::Regex;

static BASIC_PATTERN: Regex = Regex::new(r"/^[\w\-\.]+$/gm").unwrap();
static EMAIL_PATTERN: Regex = Regex::new(r"/^[\w\-\.]+@([\w-]+\.)+[\w-]{2,}$/gm").unwrap();

#[derive(Clone, Deserialize, Debug)]
pub struct CheckedString {
    str: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Email {
    str: String,
}

impl CheckedString {
    fn new(input: String) -> Option<CheckedString> {
        if input.len() > 254 {
            return None;
        }
        if EMAIL_PATTERN.is_match(&input) {
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
        if EMAIL_PATTERN.is_match(&input) {
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
        let input = String::from(v);
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
impl diesel::Expression for T {
    type SqlType = diesel::sql_types::VarChar;
}
