use gekraftet_core::maths::{ Vector3I, Vector3F };
use gekraftet_core::world::{ self, Chunk };
use crate::mesh::{ Face, Mesh, MeshBuilder };
use super::{ Mesher, BLOCK_LENGTH };

pub struct BasicFaceMesher<'a> {
    chunk: &'a Chunk,
}

impl<'a> BasicFaceMesher<'a> {
    fn intrasection_cull(&self) -> Mesh {
        let mut mb = MeshBuilder::new();
        
        for (i, sec) in self.chunk.sections().iter().enumerate() {
            let range = (0..world::SECTION_LENGTH_X)
                .flat_map(move |x| (0..world::SECTION_LENGTH_Z)
                    .flat_map(move |z| (0..world::SECTION_LENGTH_Y)
                        .map(move |y| (x, z, y))
                ));
            
            for (x, z, y) in range {
                let block = &sec[x][z][y];

                // Otherwise debug builds will panic with integer underflow.
                let px = x + 1;
                let mx = x.wrapping_sub(1);
                let py = y + 1;
                let my = y.wrapping_sub(1);
                let pz = z + 1;
                let mz = z.wrapping_sub(1);
                
                let block_right = sec.get(px).map(|b| &b[z][y]);
                let block_left  = sec.get(mx).map(|b| &b[z][y]);
                let block_front  = sec[x].get(pz).map(|b| &b[y]);
                let block_back   = sec[x].get(mz).map(|b| &b[y]);
                let block_top    = sec[x][z].get(py);
                let block_bottom = sec[x][z].get(my);

                let (x, y, z) = (x as i32, y as i32, z as i32);

                let pos = Vector3I::new(
                    x + self.chunk.position().x() * world::CHUNK_LENGTH_X as i32,
                    y + self.chunk.position().y() * world::CHUNK_LENGTH_Y as i32 + (i * world::SECTION_LENGTH_X) as i32,
                    z + self.chunk.position().z() * world::CHUNK_LENGTH_Z as i32
                );

                let origin = Vector3F::from(pos) * BLOCK_LENGTH;

                // basic culling
                let mut faces = Face::all();
                if block_left.map_or(false, |b| b.id > 0) { faces.disable(Face::LEFT) };
                if block_right.map_or(false, |b| b.id > 0) { faces.disable(Face::RIGHT) };
                if block_top.map_or(false, |b| b.id > 0) { faces.disable(Face::TOP) };
                if block_bottom.map_or(false, |b| b.id > 0) { faces.disable(Face::BOTTOM) };
                if block_front.map_or(false, |b| b.id > 0) { faces.disable(Face::FRONT) };
                if block_back.map_or(false, |b| b.id > 0) { faces.disable(Face::BACK) };

                if block.id > 0 {
                    mb = mb.add_mesh(MeshBuilder::create_cube(BLOCK_LENGTH, origin, faces));
                }
            };
        }

        mb.build()
    }
}

impl<'a> Mesher<'a> for BasicFaceMesher<'a> {
    fn from_chunk(chunk: &'a Chunk) -> Self {
        Self {
            chunk
        }
    }

    fn generate_mesh(&self) -> Mesh {
        self.intrasection_cull()
    }
}
