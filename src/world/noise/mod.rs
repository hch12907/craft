mod perlin2d;
mod perlin3d;

pub use perlin2d::Perlin2D;
pub use perlin3d::Perlin3D;

use crate::maths::Random;
use rand::SeedableRng;

pub struct PerlinOption {
    octaves: u32,
    amplitude: f32,
    frequency: f32,
    lacunarity: f32,
    persistance: f32,
}

impl PerlinOption {
    pub fn new() -> Self {
        Self {
            octaves: 1,
            amplitude: 1.0,
            frequency: 1.0,
            lacunarity: 1.0,
            persistance: 1.0,
        }
    }

    pub fn amplitude(mut self, amp: f32) -> Self {
        self.amplitude = amp;
        self
    }

    pub fn frequency(mut self, freq: f32) -> Self {
        self.frequency = freq;
        self
    }

    pub fn lacunarity(mut self, lac: f32) -> Self {
        self.lacunarity = lac;
        self
    }

    pub fn persistance(mut self, per: f32) -> Self {
        self.persistance = per;
        self
    }

    pub fn octaves(mut self, oct: u32) -> Self {
        self.octaves = oct;
        self
    }

    pub fn build_2d(self, seed: u64) -> Perlin2D {
        Perlin2D {
            octaves: self.octaves,
            amplitude: self.amplitude,
            frequency: self.frequency,
            lacunarity: self.lacunarity,
            persistance: self.persistance,
            rng: Random::seed_from_u64(seed),
        }
    }

    pub fn build_3d(self, seed: u64) -> Perlin3D {
        Perlin3D {
            octaves: self.octaves,
            amplitude: self.amplitude,
            frequency: self.frequency,
            lacunarity: self.lacunarity,
            persistance: self.persistance,
            rng: Random::seed_from_u64(seed),
        }
    }
}

// --- Helper functions start here ---
use std::ops::{ Add, Sub, Mul };

pub (in self) fn lerp<T>(x: T, y: T, t: T) -> T 
    where T: Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Copy
{
    x + t * (y - x)
}

pub (in self) fn interpolate(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0  - 15.0) + 10.0)
}
