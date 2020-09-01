use std::collections::HashMap;

use serde::{Deserialize, Deserializer};
use serde::de::Error;

use crate::ident::Identifier;
use crate::types::{Direction, DisplayTransformation, Vec3};

#[derive(Debug, Deserialize)]
pub struct Model {
    pub parent: Identifier,
    #[serde(default = "ao_default")]
    pub ambientocclusion: bool,
    #[serde(default)]
    pub display: Display,
    #[serde(default)]
    pub textures: HashMap<String, TextureRef>,
    #[serde(default)]
    pub elements: Vec<Cube>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TextureRef {
    Literal(Identifier),
    Reference(String),
}

impl TextureRef {
    pub fn resolve(self, map: &HashMap<String, TextureRef>) -> TextureRef {
        match self {
            x @ TextureRef::Literal(_) => x,
            TextureRef::Reference(target) => map.get(&target).map(|r| r.clone()).unwrap_or(TextureRef::Reference(target)),
        }
    }

    pub fn unwrap(self) -> Identifier {
        match self {
            TextureRef::Literal(id) => id,
            x @ _ => panic!("Can't unwrap {:?}", x),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Cube {
    pub from: Vec3,
    pub to: Vec3,
    pub rotation: Option<Rotation>,
    #[serde(default = "shade_default")]
    pub shade: bool,
    pub faces: Faces,
}

#[derive(Debug, Deserialize)]
pub struct Rotation {
    pub origin: Vec3,
    pub axis: RotationAxis,
    pub angle: f32,
    #[serde(default)]
    pub rescale: bool,
}

#[derive(Debug, Deserialize, Default)]
pub struct Display {
    pub thirdperson_righthand: Option<DisplayTransformation>,
    pub thirdperson_lefthand: Option<DisplayTransformation>,
    pub firstperson_righthand: Option<DisplayTransformation>,
    pub firstperson_lefthand: Option<DisplayTransformation>,
    pub gui: Option<DisplayTransformation>,
    pub head: Option<DisplayTransformation>,
    pub ground: Option<DisplayTransformation>,
    pub fixed: Option<DisplayTransformation>,
}

impl Display {
    pub fn merge(self, overrides: Display) -> Display {
        Display {
            thirdperson_righthand: overrides.thirdperson_righthand.or(self.thirdperson_righthand),
            thirdperson_lefthand: overrides.thirdperson_lefthand.or(self.thirdperson_lefthand),
            firstperson_righthand: overrides.firstperson_righthand.or(self.firstperson_righthand),
            firstperson_lefthand: overrides.firstperson_lefthand.or(self.firstperson_lefthand),
            gui: overrides.gui.or(self.gui),
            head: overrides.head.or(self.head),
            ground: overrides.ground.or(self.ground),
            fixed: overrides.fixed.or(self.fixed),
        }
    }
}

impl From<Display> for crate::types::Display {
    fn from(d: Display) -> Self {
        crate::types::Display {
            thirdperson_righthand: d.thirdperson_righthand.unwrap_or_default(),
            thirdperson_lefthand: d.thirdperson_lefthand.unwrap_or_default(),
            firstperson_righthand: d.firstperson_righthand.unwrap_or_default(),
            firstperson_lefthand: d.firstperson_lefthand.unwrap_or_default(),
            gui: d.gui.unwrap_or_default(),
            head: d.head.unwrap_or_default(),
            ground: d.ground.unwrap_or_default(),
            fixed: d.fixed.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Faces {
    pub down: Option<Face>,
    pub up: Option<Face>,
    pub north: Option<Face>,
    pub south: Option<Face>,
    pub west: Option<Face>,
    pub east: Option<Face>,
}

#[derive(Debug, Deserialize)]
pub struct Face {
    pub uv: Option<[f32; 4]>,
    pub texture: TextureRef,
    pub cullface: Option<Direction>,
    #[serde(default)]
    pub rotation: f32,
    pub tintindex: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RotationAxis { X, Y, Z }

impl<'de> Deserialize<'de> for TextureRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        let mut s = String::deserialize(deserializer)?;
        if s.starts_with("#") {
            s.remove(0);
            Ok(TextureRef::Reference(s))
        } else {
            s.parse().map(TextureRef::Literal).map_err(|e| D::Error::custom(e))
        }
    }
}

fn ao_default() -> bool { true }

fn shade_default() -> bool { true }