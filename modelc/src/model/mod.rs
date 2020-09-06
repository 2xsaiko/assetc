use crate::ident::Identifier;
use crate::types::{Direction, Display, Vec2, Vec3};
use crate::vanilla::BlockStateDef;
use crate::vanilla::Model as JsonModel;

mod quadifier;

#[derive(Debug)]
pub struct Model {
   pub particle: Identifier,
   pub transformation: Display,
   pub meshes: Vec<Mesh>,
}

#[derive(Debug)]
pub struct Mesh {
    pub quads: Vec<Quad>,
}

#[derive(Debug)]
pub struct Quad {
    pub texture: Identifier,
    pub vertices: [Vertex; 4],
    pub normal: Vec3,
    pub color_index: i32,
    pub cull_face: Option<Direction>,
}

#[derive(Debug)]
pub struct Vertex {
    pub xyz: Vec3,
    pub uv: Vec2,
}

impl Model {
    pub fn from_blockstate(state: &BlockStateDef) -> Self {
        unimplemented!()
    }

    pub fn from_json_model(model: &JsonModel) -> Result<Self, ()> {
        if !model.is_fully_resolved() { return Err(()); }

        let tex = model.texture("particle").unwrap_or(Identifier::new("minecraft", "missingno"));
        let tr = model.display.clone().into();
        let mesh = quadifier::cubes_to_mesh(model.elements(), &model.textures);
        Ok(Model {
            particle: tex,
            transformation: tr,
            meshes: vec![mesh],
        })
    }
}

impl From<BlockStateDef> for Model {
    fn from(state: BlockStateDef) -> Self { Model::from_blockstate(&state) }
}

impl From<JsonModel> for Model {
    fn from(model: JsonModel) -> Self { Model::from_json_model(&model).unwrap() }
}