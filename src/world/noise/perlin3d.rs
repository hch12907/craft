use crate::maths::{ Vector3D, Vector3I, Vector3F, Random };
use crate::utils::{ lerp, fade };
use super::{ NoiseGen, NoiseGenOption };

use rand::SeedableRng;

/// A 3D Perlin noise generator. The implementation is largely similar to the
/// reference implementation by Ken Perlin
/// ([see here](https://mrl.nyu.edu/~perlin/noise/)). 
pub struct Perlin3D {
    pub octaves: u32,

    pub amplitude: f64,
    pub frequency: f32,
    pub lacunarity: f32, // lacunarity means "gap".
    pub persistance: f64,
    
    pub rng: Random,
}

const PERMUTATIONS: [i32; 512] = [
    151,160,137,91,90,15,131, 13,201,95,96, 53,194,233, 7,225,140,36,103,30,69,
    142,8,99,37,240,21,10,23, 190, 6,148,247,120,234,75,0,26,197,62,94,252,219,
    203,117,35,11,32,57,177,33,88,237,149,56,87,174,20,125,136,171,168, 68,175,
    74,165,71,134,139, 48,27,166,77,146,158,231, 83,111,229,122,60,211,133,230,
    220,105,92,41,55,46,245,40,244,102,143,54, 65,25,63,161,1,216,80,73,209,76,
    132,187,208, 89,18,169,200,196,135,130,116,188,159, 86,164,100,109,198,173,
    186,3,64,52,217,226,250,124,123,5,202,38,147,118,126,255,82,85,212,207,206,
    59,227,47,16,58,17,182,189,28,42,223,183,170,213,119,248,152, 2,44,154,163,
    70,221,153,101,155,167, 43,172, 9,129,22,39,253, 19,98,108,110, 79,113,224,
    232,178,185, 112,104,218,246,97,228,251,34,242,193,238,210,144, 12,191,179,
    162,241, 81,51,145,235,249,14,239,107,49,192,214,31,181,199,106,157,184,84,
    204,176,115,121,50,45,127,4,150,254,138,236,205,93,222,114,67,29,24,72,243,
    141,128,195,78,66,215,61,156,180,
    // repeated
    151,160,137,91,90,15,131, 13,201,95,96, 53,194,233, 7,225,140,36,103,30,69,
    142,8,99,37,240,21,10,23, 190, 6,148,247,120,234,75,0,26,197,62,94,252,219,
    203,117,35,11,32,57,177,33,88,237,149,56,87,174,20,125,136,171,168, 68,175,
    74,165,71,134,139, 48,27,166,77,146,158,231, 83,111,229,122,60,211,133,230,
    220,105,92,41,55,46,245,40,244,102,143,54, 65,25,63,161,1,216,80,73,209,76,
    132,187,208, 89,18,169,200,196,135,130,116,188,159, 86,164,100,109,198,173,
    186,3,64,52,217,226,250,124,123,5,202,38,147,118,126,255,82,85,212,207,206,
    59,227,47,16,58,17,182,189,28,42,223,183,170,213,119,248,152, 2,44,154,163,
    70,221,153,101,155,167, 43,172, 9,129,22,39,253, 19,98,108,110, 79,113,224,
    232,178,185, 112,104,218,246,97,228,251,34,242,193,238,210,144, 12,191,179,
    162,241, 81,51,145,235,249,14,239,107,49,192,214,31,181,199,106,157,184,84,
    204,176,115,121,50,45,127,4,150,254,138,236,205,93,222,114,67,29,24,72,243,
    141,128,195,78,66,215,61,156,180,
];

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
        let  a = (PERMUTATIONS[c + 0] + cube.y()) as usize;
        let aa = (PERMUTATIONS[a + 0] + cube.z()) as usize;
        let ab = (PERMUTATIONS[a + 1] + cube.z()) as usize;
        let  b = (PERMUTATIONS[c + 1] + cube.y()) as usize;
        let ba = (PERMUTATIONS[b + 0] + cube.z()) as usize;
        let bb = (PERMUTATIONS[b + 1] + cube.z()) as usize;

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

        let noise_000 = grad(PERMUTATIONS[aa+0], x,       y,       z      );
        let noise_001 = grad(PERMUTATIONS[ba+0], x - 1.0, y,       z      );
        let noise_010 = grad(PERMUTATIONS[ab+0], x,       y - 1.0, z      );
        let noise_011 = grad(PERMUTATIONS[bb+0], x - 1.0, y - 1.0, z      );
        let noise_100 = grad(PERMUTATIONS[aa+1], x,       y,       z - 1.0);
        let noise_101 = grad(PERMUTATIONS[ba+1], x - 1.0, y,       z - 1.0);
        let noise_110 = grad(PERMUTATIONS[ab+1], x,       y - 1.0, z - 1.0);
        let noise_111 = grad(PERMUTATIONS[bb+1], x - 1.0, y - 1.0, z - 1.0);

        println!("{:?}", (x, y, z));

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

impl NoiseGen<Vector3F> for Perlin3D {
    fn with_option_and_seed(option: NoiseGenOption, seed: u64) -> Self {
        Self {
            octaves: option.octaves,
            amplitude: option.amplitude,
            frequency: option.frequency,
            lacunarity: option.lacunarity,
            persistance: option.persistance,
            rng: Random::seed_from_u64(seed),
        }
    }

    fn generate_noise_at(&mut self, pos: Vector3F) -> f64 {
        let mut total = 0.0;

        for _ in 0..self.octaves {
            total += self.generate_noise(pos * self.frequency) * self.amplitude;
            self.amplitude *= self.persistance;
            self.frequency *= self.lacunarity;
        };

        total
    }
}
