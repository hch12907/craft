use super::perlin::*;
use crate::maths::*;

// use rand::distributions::Distribution;
// use rand_distr::{ UnitCircle, UnitSphere };
// use rand_pcg::Pcg64;

pub struct Noise {
    // generator: Pcg64,
    //generator: Perlin3D,
    generator: Perlin2D,
}

impl Noise {
    pub fn new(seed: u64) -> Self {
        Self {
            // generator: Pcg64::new(seed as u128, 0xa02bdbf7bb3c0a7ac28fa16a64abf96),
            generator: PerlinOption::new()
                .octaves(8)    
                .frequency(0.7)
                .amplitude(20.0)
                .persistance(1.05)
                .lacunarity(0.9)
                .build_2d(seed),
        }
    }

    pub fn generate_noise_3d(&mut self, at: Vector3F) -> f32 {
        //self.generator.generate_noise(at * 0.25)
        0.0
    }

    pub fn generate_noise_2d(&mut self, at: Vector2F) -> f32 {
        self.generator.generate_noise(at)
    }

    /*pub fn generate_noise_2d(&mut self, at: Vector2F) -> f32 {
        let grid_orig = Vector2I::from(at);
        let grid_edge = grid_orig + 1;
        let offset = at - Vector2F::from(grid_orig);

        let mut calc_noise = |at, grid_x, grid_y| {
            let grad = Vector2F::from_array(UnitCircle.sample(&mut self.generator));
            let dist = at - Vector2F::from(Vector2I::new(grid_x, grid_y));
            let dist: Vector2F = dist;
            dist.dot(grad)
        };

        // interpolate with 6t^6 - 15t^5 + 10t^3
        let calc_weight = |t| t * t * t * (t * (6.0 * t - 15.0) + 10.0);

        let lerp = |x, y, s| {
            x * (1.0 - s) + y
        };
        
        let noise_00 = calc_noise(at, grid_orig.x(), grid_orig.y());
        let noise_01 = calc_noise(at, grid_orig.x(), grid_edge.y());
        
        let noise_10 = calc_noise(at, grid_edge.x(), grid_orig.y());
        let noise_11 = calc_noise(at, grid_edge.x(), grid_edge.y());

        let weight_n = calc_weight(offset.y());
        let weight_l = calc_weight(offset.x());

        let n1 = lerp(noise_00, noise_01, weight_n);
        let n2 = lerp(noise_10, noise_11, weight_n);

        lerp(n1, n2, weight_l)
    }*/
}
