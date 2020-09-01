use std::collections::HashMap;

use crate::ident::Identifier;
use crate::types::{Direction, Display, Vec2, Vec3};
use crate::vanilla::BlockStateDef;
use crate::vanilla::Model as JsonModel;

#[derive(Debug)]
pub struct Model {
    particle: Identifier,
    transformation: Display,
    meshes: Vec<Mesh>,

}

#[derive(Debug)]
struct Mesh {
    quads: Vec<Quad>,
}

#[derive(Debug)]
struct Quad {
    texture: Identifier,
    vertices: [Vertex; 4],
    normal: Vec3,
    color_index: i32,
    cull_face: Option<Direction>,
}

#[derive(Debug)]
struct Vertex {
    xyz: Vec3,
    uv: Vec2,
}

impl Model {
    pub fn from_blockstate(state: &BlockStateDef) -> Self {
        unimplemented!()
    }

    pub fn from_json_model(model: &JsonModel) -> Self {
        model.textures.get("particle").map(|r|r.)
        unimplemented!()
    }
}

impl From<BlockStateDef> for Model {
    fn from(state: BlockStateDef) -> Self { Model::from_blockstate(&state) }
}

impl From<JsonModel> for Model {
    fn from(model: JsonModel) -> Self { Model::from_json_model(&model) }
}