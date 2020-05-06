use std::ops::{ Deref, DerefMut };
use crate::maths::{ Vector2I, Vector3I };

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlockPos(pub Vector3I);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChunkPos(pub Vector2I);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SectionPos(pub Vector3I);

impl From<BlockPos> for ChunkPos {
    fn from(b: BlockPos) -> Self {
        let x = (b.0).x() / 16;
        let z = (b.0).z() / 16;
        Self(Vector2I::new(x, z))
    }
}

impl From<BlockPos> for SectionPos {
    fn from(b: BlockPos) -> Self {
        Self(b.0 / 16)
    }
}

impl From<SectionPos> for ChunkPos {
    fn from(s: SectionPos) -> Self {
        let x = s.0.x();
        let z = s.0.z();
        Self(Vector2I::new(x, z))
    }
}

impl From<Vector3I> for BlockPos {
    fn from(s: Vector3I) -> Self {
        Self(s)
    }
}

impl From<Vector2I> for ChunkPos {
    fn from(s: Vector2I) -> Self {
        Self(s)
    }
}

impl From<Vector3I> for SectionPos {
    fn from(s: Vector3I) -> Self {
        Self(s)
    }
}

impl BlockPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(Vector3I::new(x, y, z))
    }
}

impl ChunkPos {
    pub fn new(x: i32, z: i32) -> Self {
        Self(Vector2I::new(x, z))
    }
}

impl SectionPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(Vector3I::new(x, y, z))
    }
}

impl Deref for BlockPos {
    type Target = Vector3I;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BlockPos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for ChunkPos {
    type Target = Vector2I;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ChunkPos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for SectionPos {
    type Target = Vector3I;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SectionPos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
