mod angle;
mod matrix;
mod random;
mod vector;

mod x86 {
    mod matrix_simd; // contains unsafe SIMD code, x86(_64) only
}

pub use angle::*;
pub use matrix::*;
pub use vector::*;
pub use random::*;

pub type RGBA = Vector4F;
