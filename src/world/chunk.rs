use std::mem::MaybeUninit;
use std::ops::Deref;

use crate::mesh::Mesh;
use crate::maths::{ Vector3F, Vector3I };
use crate::utils::{ lerp, PartialArray };
use super::*;

#[derive(Clone, Debug)]
pub struct Chunk {
    position: ChunkPos,
    pub(in crate::world) sections: [Section; 16], 
}

#[derive(Clone, Debug)]
pub struct Section {
    pub(in crate::world) blocks: Box<[[[Block; 16]; 16]; 16]>,
}

impl Chunk {
    pub fn new<A, G>(at: A, noise: &mut Noise<G>) -> Self 
        where A: Into<ChunkPos>,
              G: NoiseGen
    {
        let at = at.into();

        // Avoid unnecessary copies with MaybeUninit
        let mut sections = PartialArray::<Section, 16>::new();

        for i in 0..16 {
            let ChunkPos(pos) = at;
            let sect = SectionPos::new(pos.x(), i, pos.y());
            sections.push(Section::new(sect, noise)).unwrap();
        };

        Self {
            position: at,
            sections: sections.into_full_array().unwrap()
        }
    }

    pub fn position(&self) -> ChunkPos {
        self.position
    }

    pub fn generate_mesh<'a, M: Mesher<'a>>(&'a self) -> Mesh {
        let mesher = M::from_chunk(self);
        mesher.generate_mesh()
    }

    pub fn sections(&self) -> &[Section] {
        self.sections.as_ref()
    }
}

impl Deref for Section {
    type Target = [[[Block; 16]; 16]; 16];

    fn deref(&self) -> &Self::Target {
        &self.blocks
    }
}

impl Section {
    pub fn new<G>(at: SectionPos, noise: &mut Noise<G>) -> Self 
        where G: NoiseGen
    {
        let blocks: Box<[[[Block; 16]; 16]; 16]>;

        let SectionPos(pos) = at;
        let starting = pos * 16;

        let mut noises = [[[0.0; 3]; 5]; 3];
        for x in 0..=(16 / 8) {
            for y in 0..=(16 / 4) {
                for z in 0..=(16 / 8) {
                    let relative_pos = Vector3I::new(x * 8, y * 4, z * 8);
                    let actual_pos = starting + relative_pos;
                    let block_pos = Vector3F::from(actual_pos);
                    let noise = noise.generate_noise(block_pos);
                    let (x, y, z) = (x as usize, y as usize, z as usize);
                    noises[x][y][z] = noise;
                }
            }
        }

        let mut blox: Vec<[[Block; 16]; 16]> = Vec::with_capacity(16);

        for x in 0..16 {
            let mut bloy = PartialArray::<[Block; 16], 16>::new();

            for y in 0..16 {
                let mut bloz = PartialArray::<Block, 16>::new();

                for z in 0..16 {
                    let relative_pos = Vector3I::new(x as i32, y as i32, z as i32);
                    let actual_pos = starting + relative_pos;

                    let noise = {
                        let (x0, y0, z0) = (
                            x / 8 as usize, 
                            y / 4 as usize, 
                            z / 8 as usize
                        );

                        let (x1, y1, z1) = (
                            x0 + 1,
                            y0 + 1,
                            z0 + 1
                        );
                        
                        let (u, v, w) = (
                            (x & 7) as f64 / 8.0, 
                            (y & 3) as f64 / 4.0, 
                            (z & 7) as f64 / 8.0
                        );

                        let lerp00 = lerp(noises[x0][y0][z0], noises[x1][y0][z0], u);
                        let lerp01 = lerp(noises[x0][y0][z1], noises[x1][y0][z1], u);
                        let lerp10 = lerp(noises[x0][y1][z0], noises[x1][y1][z0], u);
                        let lerp11 = lerp(noises[x0][y1][z1], noises[x1][y1][z1], u);

                        let lerp0 = lerp(lerp00, lerp10, v);
                        let lerp1 = lerp(lerp01, lerp11, v);

                        lerp(lerp0, lerp1, w)
                    };

                    let id = if noise + 64.0 - actual_pos.y() as f64 > 0.0 {
                        1
                    } else {
                        0
                    };

                    bloz.push(Block::new(id)).unwrap();
                }

                bloy.push(bloz.into_full_array().unwrap()).unwrap();
            }

            blox.push(bloy.into_full_array().unwrap());
        }

        // Convert Vec<[[Block; 16]; 16]> into Box<[[[Block; 16]; 16]; 16]>
        blocks = unsafe {
            let mut blox = std::mem::ManuallyDrop::new(blox);
            assert!(blox.len() == 16, "a chunk should have 16 sections");
            let ptr = blox.as_mut_ptr() as *mut [[[Block; 16]; 16]; 16];
            Box::from_raw(ptr)
        };

        /*
        // TODO: Make things flat
        // (prerequisite: an actual working const generic system in rust)
        let range = (0..16)
            .flat_map(|x| (0..16)
                .flat_map(|y| (0..16)
                    .map(|z| (x, y, z))
            ));
        
        for (x, y, z) in range {
            let relative_pos = Vector3I::new(x, y, z);
            let actual_pos = starting + relative_pos;
            let block_pos = Vector3F::from(actual_pos);
        };
        */

        Self {
            blocks
        }
    }
}

