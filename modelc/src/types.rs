use serde::Deserialize;

pub type Vec2 = [f32; 2];

pub type Vec3 = [f32; 3];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction { Down, Up, North, South, West, East }

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Display {
    pub thirdperson_righthand: DisplayTransformation,
    pub thirdperson_lefthand: DisplayTransformation,
    pub firstperson_righthand: DisplayTransformation,
    pub firstperson_lefthand: DisplayTransformation,
    pub gui: DisplayTransformation,
    pub head: DisplayTransformation,
    pub ground: DisplayTransformation,
    pub fixed: DisplayTransformation,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct DisplayTransformation {
    rotation: Vec3,
    translation: Vec3,
    scale: Vec3,
}

impl Default for DisplayTransformation {
    fn default() -> Self {
        DisplayTransformation {
            rotation: [0.0, 0.0, 0.0],
            translation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}