use std::collections::HashMap;

use serde::{Deserialize, Deserializer};
use serde::de::Error;

use crate::ident::Identifier;
use crate::types::{Direction, DisplayTransformation, Vec3};

#[derive(Clone, Debug, Deserialize)]
pub struct Model {
    pub parent: Option<Identifier>,
    pub ambientocclusion: Option<bool>,
    #[serde(default)]
    pub display: Display,
    #[serde(default)]
    pub textures: HashMap<String, TextureRef>,
    pub elements: Option<Vec<Cube>>,
}

impl Model {
    pub fn merge(&mut self, mut parent: Model) {
        self.parent = parent.parent;
        self.ambientocclusion = self.ambientocclusion.or(parent.ambientocclusion);
        self.display.merge(parent.display);
        parent.textures.extend(self.textures.drain());
        self.textures = parent.textures;
        if self.elements.is_none() {
            self.elements = parent.elements;
        }
    }

    pub fn ambientocclusion(&self) -> bool { self.ambientocclusion.unwrap_or(true) }

    pub fn elements(&self) -> &[Cube] { self.elements.as_ref().map(|a| &**a).unwrap_or(&[]) }

    pub fn texture(&self, name: &str) -> Option<Identifier> {
        self.textures.get(name).and_then(|e| e.clone().resolve(&self.textures).literal())
    }

    pub fn is_fully_resolved(&self) -> bool { self.parent.is_none() }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TextureRef {
    Literal(Identifier),
    Reference(String),
}

impl TextureRef {
    pub fn resolve(mut self, map: &HashMap<String, TextureRef>) -> TextureRef {
        while let TextureRef::Reference(target) = &self {
            if let Some(new_t) = map.get(&*target) {
                self = new_t.clone();
            } else {
                break;
            }
        }

        self
    }

    pub fn literal(self) -> Option<Identifier> {
        match self {
            TextureRef::Literal(id) => Some(id),
            TextureRef::Reference(_) => None,
        }
    }

    pub fn unwrap(self) -> Identifier {
        match self {
            TextureRef::Literal(id) => id,
            x @ _ => panic!("Can't unwrap {:?}", x),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Cube {
    pub from: Vec3,
    pub to: Vec3,
    pub rotation: Option<Rotation>,
    #[serde(default = "shade_default")]
    pub shade: bool,
    pub faces: Faces,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Rotation {
    pub origin: Vec3,
    pub axis: RotationAxis,
    pub angle: f32,
    #[serde(default)]
    pub rescale: bool,
}

#[derive(Clone, Debug, Deserialize, Default)]
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
    pub fn merge(&mut self, parent: Display) {
        *self = Display {
            thirdperson_righthand: self.thirdperson_righthand.or(parent.thirdperson_righthand),
            thirdperson_lefthand: self.thirdperson_lefthand.or(parent.thirdperson_lefthand),
            firstperson_righthand: self.firstperson_righthand.or(parent.firstperson_righthand),
            firstperson_lefthand: self.firstperson_lefthand.or(parent.firstperson_lefthand),
            gui: self.gui.or(parent.gui),
            head: self.head.or(parent.head),
            ground: self.ground.or(parent.ground),
            fixed: self.fixed.or(parent.fixed),
        };
    }
}

impl From<Display> for crate::types::Display {
    fn from(d: Display) -> Self {
        fn adjust_pos(mut tr: DisplayTransformation) -> DisplayTransformation {
            tr.translation = [tr.translation[0] / 16.0, tr.translation[1] / 16.0, tr.translation[2] / 16.0];
            tr
        }

        // TODO apply transformations for left hand
        let thirdperson_righthand = d.thirdperson_righthand.unwrap_or_default();
        let thirdperson_lefthand = d.thirdperson_lefthand.unwrap_or(thirdperson_righthand);
        let firstperson_righthand = d.firstperson_righthand.unwrap_or_default();
        let firstperson_lefthand = d.firstperson_lefthand.unwrap_or(firstperson_righthand);

        crate::types::Display {
            thirdperson_righthand: adjust_pos(thirdperson_righthand),
            thirdperson_lefthand: adjust_pos(thirdperson_lefthand),
            firstperson_righthand: adjust_pos(firstperson_righthand),
            firstperson_lefthand: adjust_pos(firstperson_lefthand),
            gui: adjust_pos(d.gui.unwrap_or_default()),
            head: adjust_pos(d.head.unwrap_or_default()),
            ground: adjust_pos(d.ground.unwrap_or_default()),
            fixed: adjust_pos(d.fixed.unwrap_or_default()),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Faces {
    pub down: Option<Face>,
    pub up: Option<Face>,
    pub north: Option<Face>,
    pub south: Option<Face>,
    pub west: Option<Face>,
    pub east: Option<Face>,
}

impl Faces {
    pub fn iter(&self) -> FaceIter {
        FaceIter {
            inner: &self,
            next_dir: Some(Direction::Down),
        }
    }

    pub fn get_face(&self, d: Direction) -> Option<&Face> {
        match d {
            Direction::Down => self.down.as_ref(),
            Direction::Up => self.up.as_ref(),
            Direction::North => self.north.as_ref(),
            Direction::South => self.south.as_ref(),
            Direction::West => self.west.as_ref(),
            Direction::East => self.east.as_ref(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Face {
    pub uv: Option<[f32; 4]>,
    pub texture: TextureRef,
    pub cullface: Option<Direction>,
    #[serde(default)]
    pub rotation: f32,
    pub tintindex: Option<i32>,
}

impl Face {
    pub fn tintindex(&self) -> i32 { self.tintindex.unwrap_or(-1) }
}

pub struct FaceIter<'a> {
    inner: &'a Faces,
    next_dir: Option<Direction>,
}

impl<'a> Iterator for FaceIter<'a> {
    type Item = (Direction, &'a Face);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(d) = self.next_dir {
            self.next_dir = if d == Direction::East { None } else { Some(d.cycle()) };

            match self.inner.get_face(d) {
                None => {}
                Some(f) => return Some((d, f)),
            }
        }
        None
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
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

fn shade_default() -> bool { true }