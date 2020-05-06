use crate::maths::{ Vector3F, Vector2D };
use super::{ NoiseGen, NoiseGenOption };

pub struct Sine2D {
    pub octaves: u32,

    pub amplitude: f64,
    pub frequency: f32,
    pub lacunarity: f32, // lacunarity means "gap".
    pub persistance: f64,
}

impl NoiseGen for Sine2D {
    fn with_option_and_seed(option: NoiseGenOption, _seed: u64) -> Self {
        Self {
            octaves: option.octaves,
            amplitude: option.amplitude,
            frequency: option.frequency,
            lacunarity: option.lacunarity,
            persistance: option.persistance,
        }
    }

    fn generate_noise_at(&mut self, pos: Vector3F) -> f64 {
        let mut total = 0.0;

        let mut amplitude = self.amplitude;
        let mut frequency = self.frequency;

        let pos = pos.trunc2();

        for _ in 0..self.octaves {
            let pos = Vector2D::from(pos * frequency);
            let noise = 0.5 * (pos.x().sin() + pos.y().sin());

            total += noise * amplitude;

            amplitude *= self.persistance;
            frequency *= self.lacunarity;
        };

        total
    }
}
