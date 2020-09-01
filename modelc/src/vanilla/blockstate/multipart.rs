use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

use super::AppliedModel;

#[derive(Debug, Deserialize)]
pub struct Multipart(Vec<ModelPart>);

#[derive(Debug, Deserialize)]
pub struct ModelPart {
    when: Option<Predicate>,
    apply: AppliedModel,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Predicate {
    Or(OrPredicate),
    Single(PredicateData),
}

#[derive(Debug, Deserialize)]
pub struct OrPredicate {
    #[serde(rename = "OR")]
    inner: Vec<PredicateData>
}

#[derive(Debug, Deserialize)]
pub struct PredicateData(HashMap<String, VariantValues>);

#[derive(Debug)]
pub struct VariantValues(Vec<String>);

impl<'de> Deserialize<'de> for VariantValues {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Value {
            Str(String),
            Int(i32),
            Float(f32),
            Bool(bool),
        }

        let s = match Value::deserialize(deserializer)? {
            Value::Str(v) => v,
            Value::Int(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
            Value::Bool(v) => v.to_string(),
        };

        Ok(VariantValues(s.split('|').map(|s| s.to_string()).collect()))
    }
}