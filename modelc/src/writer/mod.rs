use std::collections::HashSet;
use std::io;
use std::io::Write;

use byteorder::{LittleEndian, WriteBytesExt};

use crate::ident::Identifier;
use crate::model::Model;
use crate::types::{DisplayTransformation, Vec2, Vec3};

const VERSION: u16 = 3;

pub fn write<T: Write>(model: &Model, mut target: T) -> io::Result<()> {
    // file header
    write!(target, "MCBM")?;
    target.write_u16::<LittleEndian>(VERSION)?;

    // identifier lookup table
    let mut identifiers = HashSet::new();
    identifiers.insert(&model.particle);
    for x in &model.meshes {
        for y in &x.quads {
            identifiers.insert(&y.texture);
        }
    }
    let mut identifiers: Vec<_> = identifiers.into_iter().collect();
    identifiers.sort();

    assert!(identifiers.len() <= u16::MAX as usize);
    target.write_u16::<LittleEndian>(identifiers.len() as u16)?;
    for &x in identifiers.iter() {
        write_identifier(&mut target, x)?;
    }

    target.write_u16::<LittleEndian>(identifiers.binary_search(&&model.particle).unwrap() as u16)?;

    // write transformations
    write_transformation(&mut target, &model.transformation.thirdperson_righthand)?;
    write_transformation(&mut target, &model.transformation.thirdperson_lefthand)?;
    write_transformation(&mut target, &model.transformation.firstperson_righthand)?;
    write_transformation(&mut target, &model.transformation.firstperson_lefthand)?;
    write_transformation(&mut target, &model.transformation.gui)?;
    write_transformation(&mut target, &model.transformation.head)?;
    write_transformation(&mut target, &model.transformation.ground)?;
    write_transformation(&mut target, &model.transformation.fixed)?;

    // write meshes
    assert!(model.meshes.len() <= u16::MAX as usize);
    target.write_u16::<LittleEndian>(model.meshes.len() as u16)?;
    for mesh in model.meshes.iter() {
        assert!(mesh.quads.len() <= u16::MAX as usize);
        target.write_u16::<LittleEndian>(mesh.quads.len() as u16)?;
        for quad in mesh.quads.iter() {
            target.write_u16::<LittleEndian>(identifiers.binary_search(&&quad.texture).unwrap() as u16)?;
            for x in quad.vertices.iter() {
                write_vec3_fixed_u16(&mut target, x.xyz, -1.5, 2.5)?;
                write_vec2_fixed_u16(&mut target, x.uv, -0.5, 1.5)?;
            }
            write_vec3_fixed_u16(&mut target, quad.normal, -1.0, 1.0)?;
            target.write_i32::<LittleEndian>(quad.color_index)?;
            target.write_u8(quad.cull_face.map(|d| d.index() as u8).unwrap_or(0xFF))?;
        }
    }

    Ok(())
}

fn write_transformation<T: Write>(mut target: T, t: &DisplayTransformation) -> io::Result<()> {
    write_vec3(&mut target, t.rotation)?;
    write_vec3(&mut target, t.translation)?;
    write_vec3(&mut target, t.scale)?;
    Ok(())
}

fn write_vec3<T: Write>(mut target: T, vec: Vec3) -> io::Result<()> {
    target.write_f32::<LittleEndian>(vec[0])?;
    target.write_f32::<LittleEndian>(vec[1])?;
    target.write_f32::<LittleEndian>(vec[2])?;
    Ok(())
}

fn write_vec2<T: Write>(mut target: T, vec: Vec2) -> io::Result<()> {
    target.write_f32::<LittleEndian>(vec[0])?;
    target.write_f32::<LittleEndian>(vec[1])?;
    Ok(())
}

fn write_vec3_fixed_u16<T: Write>(mut target: T, vec: Vec3, min: f32, max: f32) -> io::Result<()> {
    write_f32_fixed_u16(&mut target, vec[0], min, max)?;
    write_f32_fixed_u16(&mut target, vec[1], min, max)?;
    write_f32_fixed_u16(&mut target, vec[2], min, max)?;
    Ok(())
}

fn write_vec2_fixed_u16<T: Write>(mut target: T, vec: Vec2, min: f32, max: f32) -> io::Result<()> {
    write_f32_fixed_u16(&mut target, vec[0], min, max)?;
    write_f32_fixed_u16(&mut target, vec[1], min, max)?;
    Ok(())
}

fn write_f32_fixed_u16<T: Write>(mut target: T, f: f32, min: f32, max: f32) -> io::Result<()> {
    assert!(f >= min);
    assert!(f <= max);
    let v = ((f - min) / (max - min) * u16::MAX as f32).round() as u16;
    target.write_u16::<LittleEndian>(v)?;
    Ok(())
}

fn write_identifier<T: Write>(mut target: T, identifier: &Identifier) -> io::Result<()> {
    assert!(identifier.namespace.len() <= u16::MAX as usize);
    assert!(identifier.path.len() <= u16::MAX as usize);
    let namespace_len = if identifier.namespace == "minecraft" { u16::MAX } else { identifier.namespace.len() as u16 };
    target.write_u16::<LittleEndian>(namespace_len)?;
    target.write_u16::<LittleEndian>(identifier.path.len() as u16)?;
    if identifier.namespace != "minecraft" {
        write!(target, "{}", identifier.namespace)?;
    }
    write!(target, "{}", identifier.path)?;

    Ok(())
}