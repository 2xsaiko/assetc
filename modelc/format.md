# Data format

The file format uses little endian byte order.

## Header

 - `4D 43 42 4D` (`MCBM`) - file magic
 - version: u16 (current version is 3)

## File Structure

 - identifier table size: u16
 - \<identifier table size> identifiers
 - model particle: u16 (identifier table index)
 - thirdperson_righthand: transformation
 - thirdperson_lefthand: transformation
 - firstperson_righthand: transformation
 - firstperson_lefthand: transformation
 - gui: transformation
 - head: transformation
 - ground: transformation
 - fixed: transformation
 - mesh count: u16
 - \<mesh count> meshes
 
## Additional Data Types
 
### Identifier

 - namespace length: u16
 - path length: u16
 - namespace: n chars in utf-8 format (if length == $FFFF, no data, assume "minecraft" instead)
 - path: n chars in utf-8 format

### Transformation

 - rotation: vec3f32
 - translation: vec3f32
 - scale: vec3f32

### Mesh

 - quad count: u16
 - \<quad count> quads

### Quad

 - texture: u16 (identifier table index)
 - 4 vertices:
   - xyz: vec3u16 (range -1.5 .. 2.5 -> 0 .. 65535)
   - uv: vec2u16 (range -0.5 .. 1.5 -> 0 .. 65535)
 - normal: vec3u16 (range -1.0 .. 1.0 -> 0 .. 65535)
 - color index: i32
 - cull face: enum { down = 0, up, north, south, west, east, none = $FF)
