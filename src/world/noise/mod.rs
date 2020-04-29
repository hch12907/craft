mod generator;
mod options;
mod perlin2d;
mod perlin3d;
mod sine;

use crate::maths::*;

pub use generator::NoiseGen;
pub use options::NoiseGenOption;
pub use perlin2d::Perlin2D;
pub use perlin3d::Perlin3D;
pub use sine::*;

pub struct Noise2D<G: NoiseGen<Vector2F>> {
    generator: G,
}

impl<G: NoiseGen<Vector2F>> Noise2D<G> {
    pub fn new(seed: u64) -> Self {
        let opt = NoiseGenOption::new()
            .octaves(1)    
            .frequency(1.0)
            .amplitude(1.0)
            .persistance(1.0)
            .lacunarity(1.0);
        
        Self {
            // generator: Pcg64::new(seed as u128, 0xa02bdbf7bb3c0a7ac28fa16a64abf96),
            generator: G::with_option_and_seed(opt, seed)
        }
    }

    pub fn with_option(option: NoiseGenOption, seed: u64) -> Self {
        Self {
            generator: G::with_option_and_seed(option, seed)
        }
    }

    pub fn generate_noise(&mut self, at: Vector2F) -> f64 {
        self.generator.generate_noise_at(at)
    }
}

pub struct Noise3D<G: NoiseGen<Vector3F>> {
    generator: G,
}

impl<G: NoiseGen<Vector3F>> Noise3D<G> {
    pub fn new(seed: u64) -> Self {
        let opt = NoiseGenOption::new()
            .octaves(8)    
            .frequency(0.7)
            .amplitude(20.0)
            .persistance(1.05)
            .lacunarity(0.9);
        
        Self {
            // generator: Pcg64::new(seed as u128, 0xa02bdbf7bb3c0a7ac28fa16a64abf96),
            generator: G::with_option_and_seed(opt, seed)
        }
    }

    pub fn with_option(option: NoiseGenOption, seed: u64) -> Self {
        Self {
            generator: G::with_option_and_seed(option, seed)
        }
    }

    pub fn generate_noise(&mut self, at: Vector3F) -> f64 {
        self.generator.generate_noise_at(at)
    }
}
