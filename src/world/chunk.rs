use std::mem::MaybeUninit;
use std::ops::Deref;

use crate::mesh::Mesh;
use crate::maths::{ Vector2F, Vector3F, Vector3I };
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
        let mut sections = unsafe { 
            MaybeUninit::<[MaybeUninit<Section>; 16]>::uninit().assume_init()
        };

        for i in 0..16 {
            let ChunkPos(pos) = at;
            let sect = SectionPos::new(pos.x(), i as i32, pos.y());
            sections[i] = MaybeUninit::new(Section::new(sect, noise));
        };

        Self {
            position: at,
            sections: unsafe { std::mem::transmute(sections) },
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

        let mut blox: Vec<[[Block; 16]; 16]> = Vec::with_capacity(16);

        let mut noises = [[[0.0; 2]; 4]; 2];
        for x in 0..(16 / 8) {
            for y in 0..(16 / 4) {
                for z in 0..(16 / 8) {
                    let relative_pos = Vector3I::new(x * 8, y * 4, z * 8);
                    let actual_pos = starting + relative_pos;
                    let block_pos = Vector3F::from(actual_pos);
                    let noise = noise.generate_noise(block_pos);
                    let (x, y, z) = (x as usize, y as usize, z as usize);
                    noises[x][y][z] = noise;
                }
            }
        }

        for x in 0..16 {
            let mut bloy = PartialArray::<[Block; 16], 16>::new();

            for y in 0..16 {
                let mut bloz = PartialArray::<Block, 16>::new();

                for z in 0..16 {
                    let relative_pos = Vector3I::new(x as i32, y as i32, z as i32);
                    let actual_pos = starting + relative_pos;
                    let block_pos = Vector3F::from(actual_pos);

                    let noise = {
                        let (x, y, z) = (x / 8, y / 4, z / 8);
                        noises[x][y][z]
                    };

                    let id = if noise + 32.0 - block_pos.y() as f64 > 0.0 {
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
