use gekraftet_core::maths::*;

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vector3F,
    pub color: RGBA,
    pub texture_coord: Vector2F,
}

impl Vertex {
    pub fn new(
        pos: Vector3F,
        color: RGBA,
        t_coord: Vector2F,
    ) -> Self {
        Self {
            position: pos,
            color,
            texture_coord: t_coord
        }
    }
}
