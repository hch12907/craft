mod errors;
mod shader;
mod uniform;

use crate::windowing::Window;
use crate::mesh::Mesh;
use gl::types::*;
use shader::{ Linked, ShaderProgram };
use cgmath::Matrix4;
use std::ptr;

pub use errors::RenderError;

pub struct GlRenderer {
    projection: Matrix4<f32>,
    programs: [ShaderProgram<Linked>; 1],
    vaos: Vec<(GLuint, i32)>,
}

impl GlRenderer {
    pub fn new(ctx: &Window, proj: Matrix4<f32>) -> Self {
        gl::load_with(|s| ctx.context().get_proc_address(s) as *const _);

        let prog = ShaderProgram::new();
        let prog = prog.compile_shader(VS_SHADER, FS_SHADER).unwrap();

        Self { 
            projection: proj, 
            programs: [prog],
            vaos: Vec::new(),
        }
    }

    pub fn render_mesh(&mut self, mesh: Mesh) {
        let vao = unsafe {
            let mut vao_id: GLuint = 0;
            gl::GenVertexArrays(1, &mut vao_id as *mut _);
            gl::BindVertexArray(vao_id);
            vao_id
        };

        let _vbo = unsafe {
            let size = std::mem::size_of::<crate::mesh::Vertex>();
            let mut vbo_id: GLuint = 0;
            gl::GenBuffers(1, &mut vbo_id as *mut _);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (mesh.vertices().len() * size) as isize,
                mesh.vertices().as_ptr() as *const _, 
                gl::STATIC_DRAW
            );
            vbo_id
        };

        let _ebo = unsafe {
            let mut ebo_id: GLuint = 0;
            gl::GenBuffers(1, &mut ebo_id as *mut _);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER, 
                mesh.indices().len() as isize * 4, 
                mesh.indices().as_ptr() as *mut _, 
                gl::STATIC_DRAW
            );
            ebo_id
        };

        let _va = unsafe {
            let stride = std::mem::size_of::<crate::mesh::Vertex>() as i32;
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, stride, 12 as *const _);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, 28 as *const _);
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::EnableVertexAttribArray(2);
        };

        self.vaos.push((vao, mesh.indices().len() as i32));
    }

    pub fn change_viewport(&self, width: u32, height: u32) {
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn render(&self, time: f32, view: Matrix4<f32>) {
        unsafe {
            let model = Matrix4::from_scale(1.0f32);

            gl::ProvokingVertex(gl::LAST_VERTEX_CONVENTION);

            for p in &self.programs {
                p.use_program();
                p.use_uniform("time", &time);
                p.use_uniform("projection", &self.projection);
                p.use_uniform("view", &view);
                p.use_uniform("model", &model);

                gl::Enable(gl::DEPTH_TEST); 
                gl::Enable(gl::CULL_FACE);
                gl::ClearColor(0.45, 0.55, 0.75, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                for (vao, count) in &self.vaos {
                    gl::BindVertexArray(*vao);
                    gl::DrawElements(gl::TRIANGLES, *count as i32, gl::UNSIGNED_INT, ptr::null());
                }
            };
        }
    }
}

const VS_SHADER: &'static str = include_str!("shaders/vs.glsl");
const FS_SHADER: &'static str = include_str!("shaders/fs.glsl");
