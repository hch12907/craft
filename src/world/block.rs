use crate::maths::*;
use crate::mesh::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub metadata: u16,
    pub id: u16,
}

#[derive(Clone, Debug)]
pub struct BlockModel {
    mesh: Mesh
}

impl Block {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            metadata: 0,
        }
    }
}

impl BlockModel {
    pub fn cube() -> Self {
        Self {
            mesh: MeshBuilder::create_cube(0.25, Vector3F::new(0.0, 0.0, 0.0), Face::all())
        }
    }
}
