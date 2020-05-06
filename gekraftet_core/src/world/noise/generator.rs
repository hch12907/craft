use crate::maths::Vector3F;
use super::NoiseGenOption;

pub trait NoiseGen<Output=f64> {
    fn with_option_and_seed(option: NoiseGenOption, seed: u64) -> Self;

    fn generate_noise_at(&mut self, input: Vector3F) -> Output;
}
