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
    pub fn new<A, G>(at: A, noise: &mut Noise2D<G>) -> Self 
        where A: Into<ChunkPos>,
              G: NoiseGen<Vector2F>
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
    pub fn new<G>(at: SectionPos, noise: &mut Noise2D<G>) -> Self 
        where G: NoiseGen<Vector2F>
    {
        let blocks: Box<[[[Block; 16]; 16]; 16]>;

        let SectionPos(pos) = at;
        let starting = pos * 16;

        let mut blox: Vec<[[Block; 16]; 16]> = Vec::with_capacity(16);

        //let mut noises = [[[0.0; 2 + 1]; 4 + 1]; 2 + 1];
        //for x in 0..=(16 / 8) {
        //    for y in 0..=(16 / 4) {
        //        for z in 0..=(16 / 8) {
        //            let relative_pos = Vector3I::new(x * 8, y * 4, z * 8);
        //            let actual_pos = starting + relative_pos;
        //            let block_pos = Vector3F::from(actual_pos);
        //            let noise = noise.generate_noise_3d(block_pos);
        //            let (x, y, z) = (x as usize, y as usize, z as usize);
        //            noises[x][y][z] = noise;
        //        }
        //    }
        //}

        /*let mut noises = [[0.0; 4 + 1]; 4 + 1];
        for x in 0..=(16 / 4) {
            for y in 0..=(16 / 4) {
                let relative_pos = Vector3I::new(x * 4, y * 4, 0);
                let actual_pos = starting + relative_pos;
                let block_pos = Vector3F::from(actual_pos).trunc2();
                let noise = noise.generate_noise(block_pos);
                let (x, y) = (x as usize, y as usize);
                noises[x][y] = noise;
            }
        }*/

        for x in 0..16 {
            let mut bloy = PartialArray::<[Block; 16], 16>::new();

            for y in 0..16 {
                let mut bloz = PartialArray::<Block, 16>::new();

                for z in 0..16 {
                    let relative_pos = Vector3I::new(x, y, z);
                    let actual_pos = starting + relative_pos;
                    let block_pos = Vector3F::from(actual_pos);

                    /*let noise = {
                        // 16.0 * (block_pos.x() * 0.025).sin().abs() +
                        // 16.0 * (block_pos.z() * 0.025).sin().abs() +
                        // 32.0
                        
                        let weight_factor = 1.0 / 16.0;
                        let weight_x = weight_factor * x as f64;
                        //let weight_y = weight_factor * y as f64;
                        let weight_z = weight_factor * z as f64;

                        let (x, y, z) = (x as usize >> 4, y as usize >> 2, z as usize >> 4);

                        //let lerp1 = lerp(noises[x][y][z], noises[x + 1][y][z], weight_x);
                        //let lerp2 = lerp(noises[x][y][z], noises[x][y + 1][z], weight_y);
                        //let lerp3 = lerp(noises[x][y][z], noises[x][y][z + 1], weight_z);
                        let lerp1 = lerp(noises[x][z], noises[x + 1][z], weight_x);
                        let lerp2 = lerp(noises[x][z], noises[x][z + 1], weight_z);

                        //0.333333 * (lerp1 + lerp2 + lerp3)
                        0.5 * (lerp1 + lerp2)
                    };*/

                    let noise = {
                        let block_pos = block_pos * 0.125;
                        noise.generate_noise(block_pos.shuffle([0, 2, 1]).trunc2())
                    };

                    let id = if 32.0 * noise + 64.0 > block_pos.y() as f64 {
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
