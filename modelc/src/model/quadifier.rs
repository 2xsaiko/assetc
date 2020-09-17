use std::collections::HashMap;

use crate::model::{Mesh, Quad, Vertex};
use crate::types::{Direction, Vec2, Vec3};
use crate::vanilla::model::{Cube, TextureRef};

pub fn cubes_to_mesh(cubes: &[Cube], textures: &HashMap<String, TextureRef>) -> Mesh {
    let mut quads = vec![];

    for cube in cubes {
        for (d, face) in cube.faces.iter() {
            let uvs = face.uv.unwrap_or_else(|| {
                let a = match d {
                    Direction::Down | Direction::Up => [cube.from[0], cube.from[2], cube.to[0], cube.to[2]],
                    Direction::North | Direction::South => [cube.from[0], cube.from[1], cube.to[0], cube.to[1]],
                    Direction::West | Direction::East => [cube.from[2], cube.from[1], cube.to[2], cube.to[1]],
                };
                a
            });

            // rotate texture
            let [left, bottom, right, top] = uvs;
            let width = right - left;
            let height = top - bottom;
            let epsilon = 0.01;
            let rot = face.rotation.rem_euclid(360.0);
            let (sin, cos) = if rot.abs() < epsilon {
                (0.0, 1.0)
            } else if (rot - 90.0).abs() < epsilon {
                (1.0, 0.0)
            } else if (rot - 180.0).abs() < epsilon {
                (0.0, -1.0)
            } else if (rot - 270.0).abs() < epsilon {
                (-1.0, 0.0)
            } else {
                rot.to_radians().sin_cos()
            };

            println!("rotation: {} ({}, {})", face.rotation, sin, cos);

            let transform = |[x, y]: [f32; 2]| {
                // normalize coordinates to [-1.0, 1.0]
                let [x, y] = [
                    (x - left) / width * 2.0 - 1.0,
                    (y - bottom) / height * 2.0 - 1.0
                ];
                // rotate
                let [x, y] = [
                    x * cos + y * -sin,
                    x * sin + y * cos,
                ];
                // rescale to original dimensions
                let [x, y] = [
                    ((x + 1.0) / 2.0 * width) + left,
                    ((y + 1.0) / 2.0 * height) + bottom
                ];
                [x, y]
            };

            // get uvs for each vertex
            let uvs = [
                transform([uvs[0], uvs[1]]),
                transform([uvs[0], uvs[3]]),
                transform([uvs[2], uvs[3]]),
                transform([uvs[2], uvs[1]]),
            ];

            let xyzs = match d {
                Direction::Down => [
                    [cube.from[0], cube.from[1], cube.to[2]],
                    [cube.from[0], cube.from[1], cube.from[2]],
                    [cube.to[0], cube.from[1], cube.from[2]],
                    [cube.to[0], cube.from[1], cube.to[2]],
                ],
                Direction::Up => [
                    [cube.from[0], cube.to[1], cube.from[2]],
                    [cube.from[0], cube.to[1], cube.to[2]],
                    [cube.to[0], cube.to[1], cube.to[2]],
                    [cube.to[0], cube.to[1], cube.from[2]],
                ],
                Direction::North => [
                    [cube.to[0], cube.to[1], cube.from[2]],
                    [cube.to[0], cube.from[1], cube.from[2]],
                    [cube.from[0], cube.from[1], cube.from[2]],
                    [cube.from[0], cube.to[1], cube.from[2]],
                ],
                Direction::South => [
                    [cube.from[0], cube.to[1], cube.to[2]],
                    [cube.from[0], cube.from[1], cube.to[2]],
                    [cube.to[0], cube.from[1], cube.to[2]],
                    [cube.to[0], cube.to[1], cube.to[2]],
                ],
                Direction::West => [
                    [cube.from[0], cube.to[1], cube.from[2]],
                    [cube.from[0], cube.from[1], cube.from[2]],
                    [cube.from[0], cube.from[1], cube.to[2]],
                    [cube.from[0], cube.to[1], cube.to[2]],
                ],
                Direction::East => [
                    [cube.to[0], cube.to[1], cube.to[2]],
                    [cube.to[0], cube.from[1], cube.to[2]],
                    [cube.to[0], cube.from[1], cube.from[2]],
                    [cube.to[0], cube.to[1], cube.from[2]],
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