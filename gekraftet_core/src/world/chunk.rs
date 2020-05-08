use std::ops::Deref;
use crate::maths::{ Vector3F, Vector3I };
use crate::utils::{ lerp, PartialArray };
use super::*;

#[derive(Clone, Debug)]
pub struct Chunk {
    position: ChunkPos,
    sections: [Section; CHUNK_LENGTH_Y / SECTION_LENGTH_Y], 
}

#[derive(Clone, Debug)]
pub struct Section {
    blocks: Box<[[[Block; SECTION_LENGTH_Y]; SECTION_LENGTH_X]; SECTION_LENGTH_Z]>,
}

impl Chunk {
    pub fn new<A, G>(at: A, noise: &mut Noise<G>) -> Self 
        where A: Into<ChunkPos>,
              G: NoiseGen
    {
        let at = at.into();

        // Avoid unnecessary copies with MaybeUninit
        let mut sections = PartialArray::<Section, 16>::new();

        for i in 0..(CHUNK_LENGTH_Y / SECTION_LENGTH_Y) as i32 {
            let ChunkPos(pos) = at;
            let sect = SectionPos::new(pos.x(), pos.y() * 16 + i, pos.z());
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

    pub fn sections(&self) -> &[Section] {
        self.sections.as_ref()
    }
}

impl Deref for Section {
    type Target = [[[Block; SECTION_LENGTH_Y]; SECTION_LENGTH_Z]; SECTION_LENGTH_X];

    fn deref(&self) -> &Self::Target {
        &self.blocks
    }
}

impl Section {
    pub fn new<G>(at: SectionPos, noise: &mut Noise<G>) -> Self 
        where G: NoiseGen
    {
        let SectionPos(pos) = at;
        let starting = pos * SECTION_LENGTH_X as i32;

        let mut noises = [[[0.0; NOISE_SAMPLES_X + 1]; NOISE_SAMPLES_Y + 1]; NOISE_SAMPLES_Z + 1];
        
        for y in 0..=NOISE_SAMPLES_Y as i32 {
            for z in 0..=NOISE_SAMPLES_Z as i32 {
                for x in 0..=NOISE_SAMPLES_X as i32 {
                    let relative_pos = Vector3I::new(
                        x * NOISE_FACTOR_X as i32,
                        y * NOISE_FACTOR_Y as i32,
                        z * NOISE_FACTOR_Z as i32
                    );

                    let block_pos = Vector3F::from(starting + relative_pos);
                    let noise = noise.generate_noise(block_pos);
                    let (x, y, z) = (x as usize, y as usize, z as usize);
                    noises[x][y][z] = noise;
                }
            }
        }

        let mut bloy: Vec<[[Block; SECTION_LENGTH_X]; SECTION_LENGTH_Z]> = Vec::with_capacity(SECTION_LENGTH_Y);

        for y in 0..SECTION_LENGTH_Y {
            let mut blox = PartialArray::<[Block; SECTION_LENGTH_X], SECTION_LENGTH_Z>::new();

            for z in 0..SECTION_LENGTH_Z {
                let mut bloz = PartialArray::<Block, SECTION_LENGTH_X>::new();

                for x in 0..SECTION_LENGTH_X {
                    let relative_pos = Vector3I::new(x as i32, y as i32, z as i32);
                    let actual_pos = starting + relative_pos;

                    let noise = {
                        let (x0, y0, z0) = (
                            x / NOISE_FACTOR_X as usize, 
                            y / NOISE_FACTOR_Z as usize, 
                            z / NOISE_FACTOR_Z as usize
                        );

                        let (x1, y1, z1) = (
                            x0 + 1,
                            y0 + 1,
                            z0 + 1
                        );
                        
                        let (u, v, w) = (
                            (x % NOISE_FACTOR_X) as f64 / NOISE_FACTOR_X as f64, 
                            (y % NOISE_FACTOR_Y) as f64 / NOISE_FACTOR_Y as f64, 
                            (z % NOISE_FACTOR_Z) as f64 / NOISE_FACTOR_Z as f64
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

                blox.push(bloz.into_full_array().unwrap()).unwrap();
            }

            bloy.push(blox.into_full_array().unwrap());
        }

        // Convert Vec<[[Block; 16]; 16]> into Box<[[[Block; 16]; 16]; 16]>
        let blocks = unsafe {
            let mut bloy = std::mem::ManuallyDrop::new(bloy);
            assert!(
                bloy.len() == CHUNK_LENGTH_Y / SECTION_LENGTH_Y, 
                "a chunk should have {} sections",
                CHUNK_LENGTH_Y / SECTION_LENGTH_Y
            );
            let ptr = bloy.as_mut_ptr() as *mut _;
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

