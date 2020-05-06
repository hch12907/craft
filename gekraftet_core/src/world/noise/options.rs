pub struct NoiseGenOption {
    pub(in super) octaves: u32,
    pub(in super) amplitude: f64,
    pub(in super) frequency: f32,
    pub(in super) lacunarity: f32,
    pub(in super) persistance: f64,
}

impl NoiseGenOption {
    pub fn new() -> Self {
        Self {
            octaves: 1,
            amplitude: 1.0,
            frequency: 1.0,
            lacunarity: 1.0,
            persistance: 1.0,
        }
    }

    pub fn amplitude(mut self, amp: f64) -> Self {
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

    pub fn persistance(mut self, per: f64) -> Self {
        self.persistance = per;
        self
    }

    pub fn octaves(mut self, oct: u32) -> Self {
        self.octaves = oct;
        self
    }
}
