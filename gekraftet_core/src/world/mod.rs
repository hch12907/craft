mod block;
mod chunk;
mod noise;
mod position;

pub use block::*;
pub use chunk::*;
pub use position::*;
pub use noise::*;

pub const CHUNK_LENGTH_X: usize = 16;
pub const CHUNK_LENGTH_Y: usize = 256;
pub const CHUNK_LENGTH_Z: usize = 16;
pub const SECTION_LENGTH_X: usize = 16;
pub const SECTION_LENGTH_Y: usize = 16;
pub const SECTION_LENGTH_Z: usize = 16;

// This is used for world generation - for X it means 2 samples for every
// SECTION_LENGTH_X blocks. The samples are then interpolated using trilinear
// interpolation.
pub const NOISE_SAMPLES_X: usize = 2;
pub const NOISE_SAMPLES_Y: usize = 4;
pub const NOISE_SAMPLES_Z: usize = 2;
pub const NOISE_FACTOR_X: usize = SECTION_LENGTH_X / NOISE_SAMPLES_X;
pub const NOISE_FACTOR_Y: usize = SECTION_LENGTH_Y / NOISE_SAMPLES_Y;
pub const NOISE_FACTOR_Z: usize = SECTION_LENGTH_Z / NOISE_SAMPLES_Z;