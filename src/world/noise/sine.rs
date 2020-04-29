use crate::maths::{ Vector2F, Vector2D };
use super::{ NoiseGen, NoiseGenOption };

pub struct SineNoise {
    pub octaves: u32,

    pub amplitude: f64,
    pub frequency: f32,
    pub lacunarity: f32, // lacunarity means "gap".
    pub persistance: f64,
}

impl NoiseGen<Vector2F> for SineNoise {
    fn with_option_and_seed(option: NoiseGenOption, _seed: u64) -> Self {
        Self {
            octaves: option.octaves,
            amplitude: option.amplitude,
            frequency: option.frequency,
            lacunarity: option.lacunarity,
            persistance: option.persistance,
        }
    }

    fn generate_noise_at(&mut self, pos: Vector2F) -> f64 {
        let mut total = 0.0;

        for _ in 0..self.octaves {
            let pos = Vector2D::from(pos * self.frequency);
            let noise = 0.5 * (pos.x().sin() + pos.y().sin());

            total += noise * self.amplitude;

            self.amplitude *= self.persistance;
            self.frequency *= self.lacunarity;
        };

        total
    }
}
