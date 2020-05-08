mod basic_face;
mod greedy_cube;

use crate::mesh::Mesh;
use gekraftet_core::world::Chunk;

pub use basic_face::BasicFaceMesher;
pub use greedy_cube::GreedyCubeMesher;

pub const BLOCK_LENGTH: f32 = 0.25;

/// A trait implemented by mesh generators.
pub trait Mesher<'a> {
    fn from_chunk(chunk: &'a Chunk) -> Self;

    fn generate_mesh(&self) -> Mesh;
}
