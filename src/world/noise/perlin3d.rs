use crate::maths::{ Vector3D, Vector3I, Vector3F, Random };
use crate::utils::{ lerp, fade };
use super::{ NoiseGen, NoiseGenOption };

use rand::{ Rng, SeedableRng };

/// A 3D Perlin noise generator. The implementation is largely similar to the
/// reference implementation by Ken Perlin
/// ([see here](https://mrl.nyu.edu/~perlin/noise/)). 
pub struct Perlin3D {
    pub octaves: u32,

    pub amplitude: f64,
    pub frequency: f32,
    pub lacunarity: f32, // lacunarity means "gap".
    pub persistance: f64,
    
    permutations: Box<[u8; 512]>,
}

impl Perlin3D {
    fn generate_noise(&mut self, pos: Vector3F) -> f64 {
        //let random = self.rng.next_u64() as f64 / (1u64 << 54) as f64;
        let grid = Vector3I::from(pos);
        let cube = grid & 255;
        let relative = Vector3D::from(pos) - Vector3D::from(grid);

        let weight_n = fade(relative.z());
        let weight_m = fade(relative.y());
        let weight_l = fade(relative.x());

        let  c = cube.x() as usize;
        let  a = (self.permutations[c + 0] as i32 + cube.y()) as usize;
        let aa = (self.permutations[a + 0] as i32 + cube.z()) as usize;
        let ab = (self.permutations[a + 1] as i32 + cube.z()) as usize;
        let  b = (self.permutations[c + 1] as i32 + cube.y()) as usize;
        let ba = (self.permutations[b + 0] as i32 + cube.z()) as usize;
        let bb = (self.permutations[b + 1] as i32 + cube.z()) as usize;

        let grad = |hash, x: f64, y, z| -> f64 {
            let hash = hash & 15;
            let u = if hash < 8 { x } else { y };
            let v = if hash < 4 { 
                y 
            } else {
                if hash == 12 || hash == 14 { 
                    x 
                } else { 
                    z 
                }
            };
            let r0 = if hash & 1 == 0 { u } else { -u };
            let r1 = if hash & 2 == 0 { v } else { -v };
            r0 + r1
        };

        let (x, y, z) = (relative.x(), relative.y(), relative.z());

        let noise_000 = grad(self.permutations[aa+0], x,       y,       z      );
        let noise_001 = grad(self.permutations[ba+0], x - 1.0, y,       z      );
        let noise_010 = grad(self.permutations[ab+0], x,       y - 1.0, z      );
        let noise_011 = grad(self.permutations[bb+0], x - 1.0, y - 1.0, z      );
        let noise_100 = grad(self.permutations[aa+1], x,       y,       z - 1.0);
        let noise_101 = grad(self.permutations[ba+1], x - 1.0, y,       z - 1.0);
        let noise_110 = grad(self.permutations[ab+1], x,       y - 1.0, z - 1.0);
        let noise_111 = grad(self.permutations[bb+1], x - 1.0, y - 1.0, z - 1.0);

        let n1 = lerp(noise_000, noise_001, weight_n);
        let n2 = lerp(noise_010, noise_011, weight_n);
        let n3 = lerp(noise_100, noise_101, weight_n);
        let n4 = lerp(noise_110, noise_111, weight_n);

        let m1 = lerp(n1, n2, weight_m);
        let m2 = lerp(n3, n4, weight_m);

        let result = lerp(m1, m2, weight_l);
        result
    }
}

impl NoiseGen for Perlin3D {
    fn with_option_and_seed(option: NoiseGenOption, seed: u64) -> Self {
        let mut rng = Random::seed_from_u64(seed);
        
        let mut permutations = Box::new([0; 512]);
        for i in 0..256 {
            permutations[i] = rng.gen::<u8>();
            permutations[i + 256] = rng.gen::<u8>();
        };
        
        Self {
            octaves: option.octaves,
            amplitude: option.amplitude,
            frequency: option.frequency,
            lacunarity: option.lacunarity,
            persistance: option.persistance,
            permutations,
        }
    }

    fn generate_noise_at(&mut self, pos: Vector3F) -> f64 {
        let mut total = 0.0;

        let mut amplitude = self.amplitude;
        let mut frequency = self.frequency;

        for _ in 0..self.octaves {
            total += self.generate_noise(pos * frequency) * amplitude;
            amplitude *= self.persistance;
            frequency *= self.lacunarity;
        };

        total
    }
}
