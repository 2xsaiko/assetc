use serde::Deserialize;

pub type Vec2 = [f32; 2];

pub type Vec3 = [f32; 3];

#[derive(Copy, Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction { Down, Up, North, South, West, East }

impl Direction {
    pub const fn cycle(self) -> Self {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::North,
            Direction::North => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::East,
            Direction::East => Direction::Down,
        }
    }

    pub const fn vector(self) -> Vec3 {
        match self {
            Direction::Down => [0.0, -1.0, 0.0],
            Direction::Up => [0.0, 1.0, 0.0],
            Direction::North => [-1.0, 0.0, 0.0],
            Direction::South => [1.0, 0.0, 0.0],
            Direction::West => [0.0, 0.0, -1.0],
            Direction::East => [0.0, 0.0, 1.0],
        }
    }

    pub const fn index(self) -> usize {
        match self {
            Direction::Down => 0,
            Direction::Up => 1,
            Direction::North => 2,
            Direction::South => 3,
            Direction::West => 4,
            Direction::East => 5,
        }
    }

    pub const fn negative_axis(self) -> bool { self.index() % 2 == 0 }
}

#[derive(Clone, Debug, Deserialize, Default)]
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

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(default)]
pub struct DisplayTransformation {
    pub rotation: Vec3,
    pub translation: Vec3,
    pub scale: Vec3,
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