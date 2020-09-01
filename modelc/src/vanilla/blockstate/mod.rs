use serde::Deserialize;
use crate::ident::Identifier;

pub mod variants;
pub mod multipart;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockStateDef {
    Variants(variants::Variants),
    Multipart(multipart::Multipart),
}

#[derive(Debug, Deserialize)]
pub struct AppliedModel {
    model: Identifier,
    #[serde(default)]
    x: f32,
    #[serde(default)]
    y: f32,
    #[serde(default)]
    uvlock: bool,
}
