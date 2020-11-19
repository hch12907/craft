use std::ops::{ Deref, DerefMut };
use cgmath::{ Point2, Point3 };

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlockPos(pub Point3<i32>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChunkPos(pub Point3<i32>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SectionPos(pub Point3<i32>);

impl From<BlockPos> for ChunkPos {
    fn from(b: BlockPos) -> Self {
        let x = (b.0).x / 16;
        let y = (b.0).y / 256;
        let z = (b.0).z / 16;
        Self(Point3::<i32>::new(x, y, z))
    }
}

impl From<BlockPos> for SectionPos {
    fn from(b: BlockPos) -> Self {
        Self(b.0 / 16)
    }
}

impl From<SectionPos> for ChunkPos {
    fn from(s: SectionPos) -> Self {
        let x = s.0.x;
        let y = s.0.y / 16;
        let z = s.0.z;
        Self(Point3::<i32>::new(x, y, z))
    }
}

impl From<Point3<i32>> for BlockPos {
    fn from(s: Point3<i32>) -> Self {
        Self(s)
    }
}

impl From<Point3<i32>> for ChunkPos {
    fn from(s: Point3<i32>) -> Self {
        Self(s)
    }
}

impl From<Point3<i32>> for SectionPos {
    fn from(s: Point3<i32>) -> Self {
        Self(s)
    }
}

impl BlockPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(Point3::<i32>::new(x, y, z))
    }
}

impl ChunkPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(Point3::<i32>::new(x, y, z))
    }
}

impl SectionPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(Point3::<i32>::new(x, y, z))
    }
}

impl Deref for BlockPos {
    type Target = Point3<i32>;

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
    type Target = Point3<i32>;

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
    type Target = Point3<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SectionPos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
