use crate::maths::{ Vector2D, Vector2F, Vector2I, Random };
use rand::distributions::Distribution;
use rand_distr::UnitCircle;
use super::{ lerp, interpolate };

/// A 2D Perlin noise generator. The implementation is based on the one used in
/// TrueCraft.
pub struct Perlin2D {
    pub(in super) octaves: u32,
    
    pub(in super) amplitude: f32,
    pub(in super) frequency: f32,
    pub(in super) lacunarity: f32, // lacunarity means "gap".
    pub(in super) persistance: f32,
    pub(in super) rng: Random,
}

impl Perlin2D {
    pub fn generate_noise_at(&mut self, pos: Vector2F) -> f64 {
        let pos = Vector2D::from(pos);
        let grid_orig = Vector2I::from(pos);
        let grid_edge = grid_orig + 1;
        let offset = pos - Vector2D::from(grid_orig);

        let mut calc_noise = |at, grid_x, grid_y| {
            let grad = Vector2D::from_array(UnitCircle.sample(&mut self.rng));
            let dist = at - Vector2D::from(Vector2I::new(grid_x, grid_y));
            let dist: Vector2D = dist;
            dist.dot(grad)
        };

        let noise_00 = calc_noise(pos, grid_orig.x(), grid_orig.y());
        let noise_01 = calc_noise(pos, grid_orig.x(), grid_edge.y());
        
        let noise_10 = calc_noise(pos, grid_edge.x(), grid_orig.y());
        let noise_11 = calc_noise(pos, grid_edge.x(), grid_edge.y());

        let weight_n = interpolate(offset.y());
        let weight_l = interpolate(offset.x());

        let n1 = lerp(noise_00, noise_01, weight_n);
        let n2 = lerp(noise_10, noise_11, weight_n);

        lerp(n1, n2, weight_l)
    }

    pub fn generate_noise(&mut self, pos: Vector2F) -> f32 {
        let mut total = 0.0;
        for _ in 0..self.octaves {
            total += self.generate_noise_at(pos * self.frequency) * self.amplitude as f64;
            self.amplitude *= self.persistance;
            self.frequency *= self.lacunarity;
        };
        total as f32
    }
}
