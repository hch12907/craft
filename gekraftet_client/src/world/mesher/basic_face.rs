use gekraftet_core::maths::{ Vector3I, Vector3F };
use gekraftet_core::world::Chunk;
use crate::mesh::{ Face, Mesh, MeshBuilder };
use super::Mesher;

pub struct BasicFaceMesher<'a> {
    chunk: &'a Chunk,
}

impl<'a> BasicFaceMesher<'a> {
    fn intrasection_cull(&self) -> Mesh {
        let mut mb = MeshBuilder::new();
        
        for (i, sec) in self.chunk.sections().iter().enumerate() {
            let range = (0..16)
                .flat_map(move |y| (0..16)
                    .flat_map(move |z| (0..16)
                        .map(move |x| (y, z, x))
                ));
            
            for (y, z, x) in range {
                let factor = 0.25;

                let block = &sec[y][z][x];

                // Otherwise debug builds will panic with integer underflow.
                let px = x + 1;
                let mx = x.wrapping_sub(1);
                let py = y + 1;
                let my = y.wrapping_sub(1);
                let pz = z + 1;
                let mz = z.wrapping_sub(1);
                
                let block_top    = sec.get(py).map(|b| &b[z][x]);
                let block_bottom = sec.get(my).map(|b| &b[z][x]);
                let block_front  = sec[y].get(pz).map(|b| &b[x]);
                let block_back   = sec[y].get(mz).map(|b| &b[x]);
                let block_right  = sec[y][z].get(px);
                let block_left   = sec[y][z].get(mx);

                let (x, y, z) = (x as i32, y as i32, z as i32);

                let pos = Vector3I::new(
                    x + self.chunk.position().x() * 16,
                    y + i as i32 * 16, 
                    z + self.chunk.position().y() * 16
                );

                let origin = Vector3F::from(pos) * factor;

                // basic culling
                let mut faces = Face::all();
                if block_left.map_or(false, |b| b.id > 0) { faces.disable(Face::LEFT) };
                if block_right.map_or(false, |b| b.id > 0) { faces.disable(Face::RIGHT) };
                if block_top.map_or(false, |b| b.id > 0) { faces.disable(Face::TOP) };
                if block_bottom.map_or(false, |b| b.id > 0) { faces.disable(Face::BOTTOM) };
                if block_front.map_or(false, |b| b.id > 0) { faces.disable(Face::FRONT) };
                if block_back.map_or(false, |b| b.id > 0) { faces.disable(Face::BACK) };

                if block.id > 0 {
                    mb = mb.add_mesh(MeshBuilder::create_cube(factor, origin, faces));
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
