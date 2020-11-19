use cgmath::Point3;
use super::NoiseGenOption;

pub trait NoiseGen<Output=f64> {
    fn with_option_and_seed(option: NoiseGenOption, seed: u64) -> Self;

    fn generate_noise_at(&mut self, input: Point3<f32>) -> Output;
}
