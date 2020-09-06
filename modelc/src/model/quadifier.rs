use std::collections::HashMap;

use crate::model::{Mesh, Quad, Vertex};
use crate::types::{Direction, Vec2, Vec3};
use crate::vanilla::model::{Cube, Face, TextureRef};

pub fn cubes_to_mesh(cubes: &[Cube], textures: &HashMap<String, TextureRef>) -> Mesh {
    let mut quads = vec![];

    for cube in cubes {
        for (d, face) in cube.faces.iter() {
            let uvs = face.uv.unwrap_or_else(|| {
                let mut a = match d {
                    Direction::Down | Direction::Up => [cube.from[0], cube.from[2], cube.to[0], cube.to[2]],
                    Direction::North | Direction::South => [cube.from[2], cube.from[1], cube.to[2], cube.to[1]],
                    Direction::West | Direction::East => [cube.from[0], cube.from[1], cube.to[0], cube.to[1]],
                };
                if d.negative_axis() {
                    a[0] = 16.0 - a[0];
                    a[2] = 16.0 - a[2];
                }
                a
            });
            let uvs = [
                [uvs[0], uvs[1]],
                [uvs[2], uvs[1]],
                [uvs[2], uvs[3]],
                [uvs[0], uvs[3]],
            ];

            let xyzs = match d {
                Direction::Down => [
                    [cube.from[0], cube.from[1], cube.from[2]],
                    [cube.from[0], cube.from[1], cube.to[2]],
                    [cube.to[0], cube.from[1], cube.to[2]],
                    [cube.to[0], cube.from[1], cube.from[2]],
                ],
                Direction::Up => [
                    [cube.from[0], cube.to[1], cube.from[2]],
                    [cube.to[0], cube.to[1], cube.from[2]],
                    [cube.to[0], cube.to[1], cube.to[2]],
                    [cube.from[0], cube.to[1], cube.to[2]],
                ],
                Direction::North => [
                    [cube.from[0], cube.from[1], cube.from[2]],
                    [cube.from[0], cube.to[1], cube.from[2]],
                    [cube.from[0], cube.to[1], cube.to[2]],
                    [cube.from[0], cube.from[1], cube.to[2]],
                ],
                Direction::South => [
                    [cube.to[0], cube.from[1], cube.from[2]],
                    [cube.to[0], cube.from[1], cube.to[2]],
                    [cube.to[0], cube.to[1], cube.to[2]],
                    [cube.to[0], cube.to[1], cube.from[2]],
                ],
                Direction::West => [
                    [cube.from[0], cube.from[1], cube.from[2]],
                    [cube.from[0], cube.to[1], cube.from[2]],
                    [cube.to[0], cube.to[1], cube.from[2]],
                    [cube.to[0], cube.from[1], cube.from[2]],
                ],
                Direction::East => [
                    [cube.from[0], cube.from[1], cube.to[2]],
                    [cube.to[0], cube.from[1], cube.to[2]],
                    [cube.to[0], cube.to[1], cube.to[2]],
                    [cube.from[0], cube.to[1], cube.to[2]],
                ],
            };

            let vertices = [
                Vertex { xyz: adjust_pos(xyzs[0]), uv: adjust_pos2(uvs[0]) },
                Vertex { xyz: adjust_pos(xyzs[1]), uv: adjust_pos2(uvs[1]) },
                Vertex { xyz: adjust_pos(xyzs[2]), uv: adjust_pos2(uvs[2]) },
                Vertex { xyz: adjust_pos(xyzs[3]), uv: adjust_pos2(uvs[3]) },
            ];

            quads.push(Quad {
                texture: face.texture.clone().resolve(textures).unwrap(),
                vertices,
                normal: d.vector(),
                color_index: face.tintindex(),
                cull_face: face.cullface,
            })
        }
    }

    Mesh { quads }
}

fn adjust_pos(v: Vec3) -> Vec3 { [v[0] / 16.0, v[1] / 16.0, v[2] / 16.0] }

fn adjust_pos2(v: Vec2) -> Vec2 { [v[0] / 16.0, v[1] / 16.0] }