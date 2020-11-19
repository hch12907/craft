use cgmath::{ Point2, Point3 };
use crate::RGBA;

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Point3<f32>,
    pub color: RGBA,
    pub texture_coord: Point2<f32>,
}

impl Vertex {
    pub fn new(
        pos: Point3<f32>,
        color: RGBA,
        t_coord: Point2<f32>,
    ) -> Self {
        Self {
            position: pos,
            color,
            texture_coord: t_coord
        }
    }
}
