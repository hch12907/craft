mod basic_face;
mod greedy_cube;

use crate::mesh::Mesh;
use crate::world::Chunk;

pub use basic_face::BasicFaceMesher;
pub use greedy_cube::GreedyCubeMesher;

/// A trait implemented by mesh generators.
pub trait Mesher<'a> {
    fn from_chunk(chunk: &'a Chunk) -> Self;

    fn generate_mesh(&self) -> Mesh;
}
