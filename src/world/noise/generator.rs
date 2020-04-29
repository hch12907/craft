use super::NoiseGenOption;

pub trait NoiseGen<Input, Output=f64> {
    fn with_option_and_seed(option: NoiseGenOption, seed: u64) -> Self;

    fn generate_noise_at(&mut self, input: Input) -> Output;
}
