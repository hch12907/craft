use crate::maths::{ Vector3I, Vector3F };
use crate::mesh::{ Face, Mesh, MeshBuilder };
use crate::world::Chunk;
use super::Mesher;

pub struct BasicFaceMesher<'a> {
    chunk: &'a Chunk,
}

impl<'a> BasicFaceMesher<'a> {
    fn intrasection_cull(&self) -> Mesh {
        let mut mb = MeshBuilder::new();
        
        for sec in 0..16 {
            let range = (0..16)
                .flat_map(move |x| (0..16)
                    .flat_map(move |y| (0..16)
                        .map(move |z| (x, y, z))
                ));
            
            for (x, y, z) in range {
                let factor = 0.125;

                let block = &self.chunk.sections[sec].blocks[x][y][z];

                // Otherwise debug builds will panic with integer underflow.
                let px = x + 1;
                let mx = x.wrapping_sub(1);
                let py = y + 1;
                let my = y.wrapping_sub(1);
                let pz = z + 1;
                let mz = z.wrapping_sub(1);
                
                let block_left   = self.chunk.sections[sec].blocks.get(px).map(|b| &b[y][z]);
                let block_right  = self.chunk.sections[sec].blocks.get(mx).map(|b| &b[y][z]);
                let block_top    = self.chunk.sections[sec].blocks[x].get(py).map(|b| &b[z]);
                let block_bottom = self.chunk.sections[sec].blocks[x].get(my).map(|b| &b[z]);
                let block_front  = self.chunk.sections[sec].blocks[x][y].get(pz);
                let block_back   = self.chunk.sections[sec].blocks[x][y].get(mz);

                let (x, y, z, sec) = (x as i32, y as i32, z as i32, sec as i32);

                let pos = Vector3I::new(
                    x + self.chunk.position().x() * 16,
                    y + sec * 16, 
                    z + self.chunk.position().y() * 16
                ) + Vector3I::new(0, 0 * -128, 0);

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
