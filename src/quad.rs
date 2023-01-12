
use std::ffi::c_void;

use gl::types::GLsizei;

use crate::{
    vao::{Vertex, Vao,},
    shader_program::ShaderProgram,
};

#[allow(dead_code)]
pub struct Quad {
    vertices: [Vertex; 4],
    indices: [u32; 6],
    vao: Vao,
}

impl Default for Quad {

    fn default() -> Self {

        let vertices: [Vertex; 4] = [
            Vertex{pos: (-3.,  3.), tex_coord: ( 0.,  1.)},
            Vertex{pos: ( 3.,  3.), tex_coord: ( 1.,  1.)},
            Vertex{pos: ( 3., -3.), tex_coord: ( 1.,  0.)},
            Vertex{pos: (-3., -3.), tex_coord: ( 0.,  0.)},
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

    pub fn from_vertex_positions(vpos: [(f32, f32); 4]) -> Self {

        let vertices: [Vertex; 4] = [
            Vertex{pos: vpos[0], tex_coord: ( 0.,  1.)},
            Vertex{pos: vpos[1], tex_coord: ( 1.,  1.)},
            Vertex{pos: vpos[2], tex_coord: ( 1.,  0.)},
            Vertex{pos: vpos[3], tex_coord: ( 0.,  0.)},
        ];

        Self {
            vertices,
            ..Default::default()
        }
    }

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
