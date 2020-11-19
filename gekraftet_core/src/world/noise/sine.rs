use cgmath::{ Point2, Point3 };
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

    fn generate_noise_at(&mut self, pos: Point3<f32>) -> f64 {
        let mut total = 0.0;

        let mut amplitude = self.amplitude;
        let mut frequency = self.frequency;

        let pos = Point2::new(pos.x, pos.y);

        for _ in 0..self.octaves {
            let pos = (pos * frequency).cast::<f64>().unwrap();
            let noise = 0.5 * (pos.x.sin() + pos.y.sin());

            total += noise * amplitude;

            amplitude *= self.persistance;
            frequency *= self.lacunarity;
        };

        total
    }
}
