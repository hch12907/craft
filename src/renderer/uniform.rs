use gl;

use std::ffi::CString;
use std::mem::MaybeUninit;

use crate::maths::*;
use super::shader::*;

// A handy alias for cutting down signature length
type Program = ShaderProgram<Linked>;

pub trait Uniform : Sized {
    fn get_uniform(program: &Program, location: &str) -> Option<Self>;

    fn set_uniform(&self, program: &Program, location: &str);
}

// TODO #1: proper error handling in the future? (it's sufficient *as of now*)
// TODO #2: impl_uniform_scalar! for [T; N] - waiting for const generics
macro_rules! impl_get_uniform {
    ($ty:ty) => {
        fn get_uniform(program: &Program, location: &str) -> Option<Self> {
            let loc = CString::new(location).ok()?;
            unsafe {
                let mut s = MaybeUninit::<Self>::uninit();

                let loc = gl::GetUniformLocation(
                    program.id(),
                    loc.as_bytes_with_nul().as_ptr() as *const i8
                );

                gl::GetUniformfv(
                    program.id(),
                    loc,
                    s.as_mut_ptr() as *mut _
                );

                match gl::GetError() {
                    0 => Some(s.assume_init()),
                    _ => None,
                }
            }
        }
    };
}

macro_rules! impl_uniform_scalar {
    ($sca:ty, $func:path) => {
        impl Uniform for $sca {
            impl_get_uniform!($sca);

            fn set_uniform(&self, program: &Program, location: &str) {
                [*self].as_ref().set_uniform(program, location)
            }
        }

        impl<'a> Uniform for &'a [$sca] {
            impl_get_uniform!(&'a [$sca]);

            fn set_uniform(&self, program: &Program, location: &str) {
                let loc = match CString::new(location) {
                    Ok(x) => x,
                    _ => return
                };

                unsafe {
                    let loc = gl::GetUniformLocation(
                        program.id(),
                        loc.as_bytes_with_nul().as_ptr() as *const i8
                    );

                    $func(
                        loc,
                        self.len() as i32,
                        self.as_ptr() as *const $sca
                    );

                    let error = gl::GetError();
                    if error != 0 {
                        panic!("unable to set uniform {} - got error {}", location, error);
                    }
                }
            }
        }
    };
}

macro_rules! impl_uniform_vector {
    ($vec:ty, $func:path, $($args:expr),*) => {
        impl Uniform for $vec {
            impl_get_uniform!($vec);

            fn set_uniform(&self, program: &Program, location: &str) {
                let loc = match CString::new(location) {
                    Ok(x) => x,
                    _ => return
                };
        
                unsafe {
                    let loc = gl::GetUniformLocation(
                        program.id(),
                        loc.as_bytes_with_nul().as_ptr() as *const i8
                    );
        
                    $func(
                        loc,
                        $(self[$args]),*
                    );
        
                    let error = gl::GetError();
                    if error != 0 {
                        panic!("unable to set uniform {} - got error {}", location, error);
                    }
                }
            }
        }
    };
}

macro_rules! impl_uniform_matrix {
    ($mat:ty, $func:path) => {
        impl Uniform for $mat {
            impl_get_uniform!($mat);

            fn set_uniform(&self, program: &Program, location: &str) {
                let loc = match CString::new(location) {
                    Ok(x) => x,
                    _ => return
                };
        
                unsafe {
                    let loc = gl::GetUniformLocation(
                        program.id(),
                        loc.as_bytes_with_nul().as_ptr() as *const i8
                    );
        
                    $func(
                        loc,
                        1,
                        0,
                        &self[0][0] as *const f32
                    );
        
                    if gl::GetError() != 0 {
                        panic!("unable to set uniform {}", location)
                    }
                }
            }
        }
    };
}

impl_uniform_vector!(Vector2F, gl::Uniform2f, 0, 1);
impl_uniform_vector!(Vector3F, gl::Uniform3f, 0, 1, 2);
impl_uniform_vector!(Vector4F, gl::Uniform4f, 0, 1, 2, 3);

impl_uniform_matrix!(Matrix2, gl::UniformMatrix2fv);
impl_uniform_matrix!(Matrix3, gl::UniformMatrix3fv);
impl_uniform_matrix!(Matrix4, gl::UniformMatrix4fv);

impl_uniform_scalar!(f32, gl::Uniform1fv);
impl_uniform_scalar!(f64, gl::Uniform1dv);
