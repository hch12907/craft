mod builder;
mod faces;
mod texture;
mod vertex;

pub use builder::MeshBuilder;
pub use faces::Face;
pub use texture::Texture;
pub use vertex::Vertex;

#[derive(Clone, Debug)]
pub struct Mesh {
    vertices: Box<[Vertex]>,
    indices: Box<[u32]>,
    textures: Option<Box<[Texture]>>,
}

impl Mesh {
    pub fn vertices(&self) -> &[Vertex] {
        self.vertices.as_ref()
    }

    pub fn indices(&self) -> &[u32] {
        self.indices.as_ref()
    }

    pub fn textures(&self) -> Option<&[Texture]> {
        self.textures.as_ref().map(|x| x.as_ref())
    }
}
