
use std::ffi::c_void;

use gl::types::GLsizei;

use crate::{
    vao::{Vertex, Vao,},
    shader_program::ShaderProgram,
};

pub struct Quad {
    vertices: [Vertex; 4],
    indices: [u32; 6],
    vao: Vao,
}

impl Default for Quad {

    fn default() -> Self {

        let vertices: [Vertex; 4] = [
            Vertex{pos: (-1.,  1.)},
            Vertex{pos: ( 1.,  1.)},
            Vertex{pos: ( 1., -1.)},
            Vertex{pos: (-1., -1.)},
        ];

        let indices: [u32; 6] = [0, 1, 2, 0, 2, 3];

        let vao = Vao::new(&vertices, &indices);

        Self {
            vertices,
            indices,
            vao,
        }
    }
}

impl Quad {

    pub fn render(&self, shader_program: &ShaderProgram) {

        shader_program.bind();

        self.vao.bind();

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as GLsizei,
                gl::UNSIGNED_INT,
                0 as *const c_void,
            )
        }
    }
}
