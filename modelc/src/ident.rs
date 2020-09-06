use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub struct Identifier {
    pub namespace: String,
    pub path: String,
}

impl Identifier {
    pub fn new(namespace: impl Into<String>, path: impl Into<String>) -> Self {
        Identifier {
            namespace: namespace.into(),
            path: path.into(),
        }
    }
}

impl FromStr for Identifier {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = match s.find(':') {
            None => Identifier { namespace: "minecraft".to_string(), path: s.to_string() },
            Some(i) => Identifier { namespace: s[..i].to_string(), path: s[i + 1..].to_string() },
        };

        if !is_valid_str(&id.namespace, false) {
            Err("Namespace must match [a-z0-9_.-]")
        } else if !is_valid_str(&id.path, true) {
            Err("Path must match [a-z0-9_.-/]")
        } else {
            Ok(id)
        }
    }
}

fn is_valid_str(s: &str, path: bool) -> bool {
    s.chars().all(|c| is_valid_char(c, path))
}

fn is_valid_char(c: char, path: bool) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.' || c == '-' || (path && c == '/')
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(|e| D::Error::custom(e))
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        self.to_string().serialize(serializer)
    }
}