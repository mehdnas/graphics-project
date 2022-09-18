
use std::mem;

use gl::{self, types::*};

pub struct Vertex {
    pub pos: (f32, f32),
}

pub struct Vao {
    id: GLuint,
    vbo: GLuint,
    ebo: GLuint,
}

impl Vao {
    pub fn new(vertices: &[Vertex], indices: &[u32]) -> Self {

        let mut id: GLuint = 0;
        let mut vbo: GLuint = 0;
        let mut ebo: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
            assert_ne!(id, 0);
            gl::BindVertexArray(id);

            gl::GenBuffers(1, &mut vbo);
            assert_ne!(vbo, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<Vertex>()) as GLsizeiptr,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW
            );


            gl::GenBuffers(1, &mut ebo);
            assert_ne!(ebo, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<u32>()) as GLsizeiptr,
                indices.as_ptr().cast(),
                gl::STATIC_DRAW
            );

            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                2 * 4,
                0 as *const _
            );

            gl::BindVertexArray(0);
        }

        Self {
            id,
            vbo,
            ebo,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Vao {

    fn drop(&mut self) {
        unsafe{
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteVertexArrays(1, &self.id)
        }
    }
}
