use gl::types::*;

use std::marker::PhantomData;

use super::RenderError;
pub use super::uniform::Uniform;

/// A marker trait that indicates the current status of a program.
/// The statuses are to ensure that uniforms are used *after* the linkage
/// of shaders, and since it is easy to statically know whether or not the
/// shaders are linked (through compile_shader()), this is implemented using
/// typestates.
pub trait ProgramStatus {}

pub enum Linked {}
pub enum NotLinked {}

impl ProgramStatus for Linked {}
impl ProgramStatus for NotLinked {}

/// A shader program. The programs are registered on the driver as soon as the
/// function `new()` is called. However, in order to use a shader, one still
/// needs to compile and link it first with `compile_shader()`.
pub struct ShaderProgram<S: ProgramStatus> {
    program_id: GLuint,

    _state: PhantomData<S>,
}

impl ShaderProgram<NotLinked> {
    pub fn new() -> Self {
        let id = unsafe { gl::CreateProgram() };
        Self {
            program_id: id,

            _state: PhantomData,
        }
    }

    fn raw_gl_compile_shader(&self, source: &str, ty: GLenum) 
        -> Result<GLuint, RenderError>
    {
        let shader = unsafe {
            let id = gl::CreateShader(ty);
            gl::ShaderSource(
                id, 
                1, 
                [source.as_ptr()].as_ptr() as *mut _,
                [source.len()].as_ptr() as *mut _
            );
            gl::CompileShader(id);
            id
        };

        let log = unsafe {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut log = Vec::with_capacity(len as usize);
            gl::GetShaderInfoLog(shader, len, &mut len, log.as_mut_ptr() as *mut _);
            log.set_len(len as usize);
            String::from_utf8(log).unwrap()
        };

        match log.is_empty() {
            true => unsafe {
                gl::AttachShader(self.program_id, shader);
                Ok(shader)
            },

            false => Err(RenderError::ShaderCompile(log))
        }
    }

    pub fn compile_shader(self, vs_source: &str, fs_source: &str)
        -> Result<ShaderProgram<Linked>, RenderError> 
    {
        let vs = self.raw_gl_compile_shader(vs_source, gl::VERTEX_SHADER)?;
        let fs = self.raw_gl_compile_shader(fs_source, gl::FRAGMENT_SHADER)?;

        unsafe {
            gl::LinkProgram(self.program_id);
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);
        }

        Ok(ShaderProgram::<Linked> {
            program_id: self.program_id,

            _state: PhantomData,
        })
    }
}

impl ShaderProgram<Linked> {
    pub fn id(&self) -> GLuint {
        self.program_id
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id());
        }
    }

    pub fn use_uniform<U>(&self, name: &str, uniform: &U) 
        where U: Uniform
    {
        uniform.set_uniform(self, name)
    }
}
