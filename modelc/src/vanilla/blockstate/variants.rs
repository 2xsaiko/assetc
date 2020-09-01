use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

use serde::{Deserialize, Deserializer};
use serde::de::Error;

use super::AppliedModel;

#[derive(Debug, Deserialize)]
pub struct Variants(HashMap<PropertyList, AppliedModel>);

impl Deref for Variants {
    type Target = HashMap<PropertyList, AppliedModel>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct PropertyList(Vec<AnyProperty>);

impl FromStr for PropertyList {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .try_fold(Vec::new(), |mut acc, a| a.parse().map(|v| {
                acc.push(v);
                acc
            }))
            .map(PropertyList)
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct AnyProperty {
    name: String,
    value: String,
}

impl FromStr for AnyProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match *s.splitn(2, '=').collect::<Vec<_>>() {
            [k, v] => Ok(AnyProperty { name: k.to_string(), value: v.to_string() }),
            _ => Err(format!("No = found in property spec: {}", s)),
        }
    }
}

impl<'de> Deserialize<'de> for PropertyList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}