use crate::maths::{ Vector3I, Vector3F };
use crate::mesh::{ Face, Mesh, MeshBuilder };
use crate::utils::PartialArray;
use crate::world::{ Chunk, Section };
use super::Mesher;

pub struct GreedyCubeMesher<'a> {
    chunk: &'a Chunk,
}

#[derive(Clone, Debug, Default)]
struct GroupedBlock {
    // This bitfield is filled with the following information:
    // - block extent: (x, y, z) = 12 bits (4 bits * 3) (see note 1)
    // - block type:   (indexed) = 12 bits (see note 2)
    // - block faces:            =  6 bits (one for each face)
    // - group info:         (g) =  1 bits (g = is in group)
    // ----------------------------------------------------------------
    //                     TOTAL = 31 bits (4 bytes needed)
    // 
    // NOTE #1: Since it is impossible to have 0 extent, 0 represents 16.
    //          This allows us to save one bit of memory. 
    // NOTE #2: There are 4096 *different* blocks at best in each section,
    //          thus we store an index that points to the actual block data.
    //          This way we need 12 bits only - while saving the whole block
    //          data can take up to 8 bytes of data per group!
    bitfield: u32,
}

impl GroupedBlock {
    #[inline] // since this is used here only
    fn new(block: u16) -> Self {
        // default extent: (1, 1, 1)
        let extent = 0b100010001;
        let faces = 0b111111 << 24;

        Self {
            bitfield: extent | ((block as u32) << 12) | faces
        }
    }

    fn block_id(&self) -> usize {
        ((self.bitfield >> 12) & 0xFFF) as usize
    }

    fn extent(&self) -> Vector3I {
        let x = (self.bitfield >> 8) & 0xF;
        let y = (self.bitfield >> 4) & 0xF;
        let z = (self.bitfield >> 0) & 0xF;
        
        let x = if x == 0 { 16 } else { x };
        let y = if y == 0 { 16 } else { y };
        let z = if z == 0 { 16 } else { z };

        Vector3I::new(x as i32, y as i32, z as i32)
    }

    fn is_in_group(&self) -> bool {
        (self.bitfield >> 30) & 1 == 1
    }

    fn faces(&self) -> Face {
        Face::from_bitfield((self.bitfield >> 24) as u8 & 0b111111)
    }

    fn extend_to(&mut self, x: usize, y: usize, z: usize) {
        let bits = (x & 0xF) << 8 | (y & 0xF) << 4 | (z & 0xF) << 0;
        let mask = !0xFFF;
        self.bitfield &= mask;
        self.bitfield |= bits as u32;
    }

    fn toggle_group(&mut self) {
        self.bitfield ^= 1 << 30;
    }

    fn set_faces(&mut self, face: Face) {
        let faces = (face.into_bitfield() as u32) << 24;
        let mask = !(0b111111 << 24);
        self.bitfield &= mask;
        self.bitfield |= faces;
    }
}

impl<'a> GreedyCubeMesher<'a> {
    fn intrasection_cull(
        &self,
        sect_y: usize,
        sect: &Section,
    ) -> Mesh 
    {
        let chunk_pos = self.chunk.position().0;
        let sect_pos = Vector3I::new(chunk_pos.x(), sect_y as i32, chunk_pos.y());
        let block_pos = sect_pos * 16;

        let mut blocks = Vec::with_capacity(16);
        let mut groups: [GroupedBlock; 4096] = {
            let mut g = PartialArray::<GroupedBlock, 4096>::new();

            let range = 
                (0..16)
                    .flat_map(move |y| (0..16)
                        .map(move |z| (y, z)));

            // initialization and a marking pass along x-axis
            for (y, z) in range {
                for x in 0..16 {
                    let block_id = blocks.iter().enumerate().rev().find(|b| {
                        b.1 == &&sect[y][z][x]
                    });

                    let block_id = match block_id {
                        Some((i, _)) => i as u16,
                        None => {
                            blocks.push(&sect[y][z][x]);
                            (blocks.len() - 1) as u16
                        },
                    };

                    let mut group = GroupedBlock::new(block_id);

                    if x > 0 {
                        let b = g.get_mut(y * 256 + z * 16 + x - 1).unwrap();
                        
                        let can_disable_face =
                            blocks[b.block_id()].id != 0 && 
                            blocks[group.block_id()].id != 0;

                        let mut face1 = group.faces();
                        let mut face2 = b.faces();

                        if b.block_id() == group.block_id() {
                            group.extend_to(1 + b.extent().x() as usize, 1, 1);
                            b.toggle_group();
                        } else if can_disable_face {
                            face1.disable(Face::LEFT);
                            face2.disable(Face::RIGHT);
                            group.set_faces(face1);
                            b.set_faces(face2);
                        }
                    };

                    g.push(group).unwrap();
                }
            };

            g.into_full_array().unwrap()
        };

        // marking along z-axis
        for y in 0..16 {
            for z in 0..16 {
                for x in 0..16 {
                    if z == 0 { continue };
        
                    let idx = y * 256 + z * 16 + x;
                    let idx2 = idx - 16;
        
                    if groups[idx].is_in_group() {
                        continue
                    };

                    let can_disable_face =
                        blocks[groups[idx].block_id()].id != 0 && 
                        blocks[groups[idx2].block_id()].id != 0 &&
                        groups[idx2].extent().x() >= groups[idx].extent().x();

                    if groups[idx2].is_in_group() {
                        if can_disable_face {
                            let mut face = groups[idx].faces();
                            face.disable(Face::BACK);
                            groups[idx].set_faces(face);
                        }
                        continue
                    };
                    
                    if groups[idx].extent().x() == groups[idx2].extent().x() {
                        let mut face1 = groups[idx].faces();
                        let mut face2 = groups[idx2].faces();

                        if groups[idx].block_id() == groups[idx2].block_id() {
                            groups[idx2].toggle_group();
                            
                            let orig_ext = groups[idx].extent();
                            groups[idx].extend_to(
                                orig_ext.x() as usize,
                                orig_ext.y() as usize,
                                (orig_ext.z() + groups[idx2].extent().z()) as usize,
                            );
                        } else if can_disable_face {
                            face1.disable(Face::BACK);
                            face2.disable(Face::FRONT);
                            groups[idx].set_faces(face1);
                            groups[idx2].set_faces(face2);
                        }
                    }
                }
            }
        }

        // marking along y-axis
        for y in 0..16 {
            for z in 0..16 {
                for x in 0..16 {
                    if y == 0 { continue };
        
                    let idx = y * 256 + z * 16 + x;
                    let idx2 = idx - 256;
        
                    if groups[idx].is_in_group() {
                        continue
                    };

                    let can_disable_face =
                        blocks[groups[idx].block_id()].id != 0 && 
                        blocks[groups[idx2].block_id()].id != 0 &&
                        groups[idx2].extent().x() >= groups[idx].extent().x() &&
                        groups[idx2].extent().z() >= groups[idx].extent().z();

                    if groups[idx2].is_in_group() {
                        if can_disable_face {
                            let mut face = groups[idx].faces();
                            face.disable(Face::BOTTOM);
                            groups[idx].set_faces(face);
                        }
                        continue
                    };
                    
                    if groups[idx].extent().x() == groups[idx2].extent().x() &&
                       groups[idx].extent().z() == groups[idx2].extent().z() 
                    {
                        let mut face1 = groups[idx].faces();
                        let mut face2 = groups[idx2].faces();

                        if groups[idx].block_id() == groups[idx2].block_id() {
                            groups[idx2].toggle_group();
                            
                            let orig_ext = groups[idx].extent();
                            
                            groups[idx].extend_to(
                                orig_ext.x() as usize,
                                (orig_ext.y() + groups[idx2].extent().y()) as usize,
                                orig_ext.z() as usize,
                            );
                        } else if can_disable_face {
                            face1.disable(Face::BOTTOM);
                            face2.disable(Face::TOP);
                            groups[idx].set_faces(face1);
                            groups[idx2].set_faces(face2);
                        }
                    }
                }
            }
        }

        let mut mb = MeshBuilder::new();
        
        for (pos, grp) in groups.iter().enumerate() {
            if grp.is_in_group() { 
                continue 
            };

            if blocks[grp.block_id()].id == 0 {
                continue
            };

            let y = ((pos >> 8) & 0xF) as i32;
            let z = ((pos >> 4) & 0xF) as i32;
            let x = ((pos >> 0) & 0xF) as i32;
            let extent = Vector3F::from(grp.extent());
            let origin = Vector3I::new(x, y, z) + block_pos - grp.extent();

            let mesh = MeshBuilder::create_cuboid(
                extent * 0.25, 
                (Vector3F::from(origin) + 0.5 * extent) * 0.25,
                grp.faces()
            );
            
            mb = mb.add_mesh(mesh);
        }

        mb.build()
    }
}

impl<'a> Mesher<'a> for GreedyCubeMesher<'a> {
    fn from_chunk(chunk: &'a Chunk) -> Self {
        Self {
            chunk
        }
    }

    fn generate_mesh(&self) -> Mesh {
        let mut meshes = MeshBuilder::new();
        for (i, sect) in self.chunk.sections().iter().enumerate() {
            meshes = meshes.add_mesh(self.intrasection_cull(i, sect));
        };
        meshes.build()
    }
}
