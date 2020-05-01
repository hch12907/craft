use crate::maths::{ Vector2D, Vector2F, Vector2I, Vector3F, Random };
use crate::utils::{ lerp, fade };
use super::{ NoiseGen, NoiseGenOption };

use rand::{ Rng, SeedableRng };

/// A 2D Perlin noise generator. The implementation is based on the one used in
/// TrueCraft.
pub struct Perlin2D {
    octaves: u32,
    
    amplitude: f64,
    frequency: f32,
    lacunarity: f32, // lacunarity means "gap".
    persistance: f64,
    
    permutations: Box<[u8; 512]>,
}

impl Perlin2D {
    fn generate_noise(&mut self, pos: Vector2F) -> f64 {
        let grid = Vector2I::from(pos);
        let cube = grid & 255;
        let relative = Vector2D::from(pos) - Vector2D::from(grid);

        let weight_m = fade(relative.y());
        let weight_l = fade(relative.x());

        let  c = cube.x() as usize;
        let  a = (self.permutations[c + 0] as i32 + cube.y()) as usize;
        let aa = (self.permutations[a + 0]) as usize;
        let ab = (self.permutations[a + 1]) as usize;

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

        let (x, y) = (relative.x(), relative.y());

        let noise_000 = grad(self.permutations[aa+0], x, y,       0.0 + 0.0);
        let noise_010 = grad(self.permutations[ab+0], x, y - 1.0, 0.0 + 0.0);
        let noise_100 = grad(self.permutations[aa+1], x, y,       0.0 - 1.0);
        let noise_110 = grad(self.permutations[ab+1], x, y - 1.0, 0.0 - 1.0);

        let m1 = lerp(noise_000, noise_010, weight_m);
        let m2 = lerp(noise_100, noise_110, weight_m);

        let result = lerp(m1, m2, weight_l);
        result
    }
}

impl NoiseGen for Perlin2D {
    fn with_option_and_seed(option: NoiseGenOption, seed: u64) -> Self {
        let mut rng = Random::seed_from_u64(seed);
        
        let mut permutations = Box::new([0; 512]);
        for i in 0..256 {
            permutations[i] = rng.gen::<u8>();
            permutations[i + 256] = permutations[i];
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
        
        let pos = pos.trunc2();

        for _ in 0..self.octaves {
            total += self.generate_noise(pos * frequency) * amplitude;
            amplitude *= self.persistance;
            frequency *= self.lacunarity;
        };

        total
    }
}
