use std::{fs, convert::TryInto, thread::panicking};

use gl::{types::{GLuint, GLint, GLsizei}, UseProgram};

const INFO_LOG_BUFFER_LEN: GLsizei = 1024;


pub struct ShaderProgram {
    vertex_shader_id: GLuint,
    fragment_shader_id: GLuint,
    program_id: GLuint,
}

impl ShaderProgram {

    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> Self {

        let mut vertex_shader_id: GLuint = 0;
        let mut fragment_shader_id: GLuint = 0;
        let mut program_id: GLuint = 0;

        let vertex_shader_src = fs::read_to_string(vertex_shader_path)
            .expect("Could not read the vertex shader file");
        let fragment_shader_src = fs::read_to_string(fragment_shader_path)
            .expect("Could not read the fragment shader file");

        unsafe {
            vertex_shader_id = gl::CreateShader(gl::VERTEX_SHADER);
            assert_ne!(vertex_shader_id, 0);
            fragment_shader_id = gl::CreateShader(gl::FRAGMENT_SHADER);
            assert_ne!(fragment_shader_id, 0);
        }

        Self::compile_shader(vertex_shader_id, &vertex_shader_src);
        Self::compile_shader(fragment_shader_id, &fragment_shader_src);

        unsafe {
            program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader_id);
            gl::AttachShader(program_id, fragment_shader_id);
            gl::LinkProgram(program_id);

            let mut success = 0;
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);

            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(INFO_LOG_BUFFER_LEN as usize);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(
                    program_id,
                    INFO_LOG_BUFFER_LEN,
                    &mut log_len,
                    v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }
        }

        Self {
            vertex_shader_id,
            fragment_shader_id,
            program_id,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    fn compile_shader(shader_id: GLuint, shader_src: &String) {

        unsafe {
            gl::ShaderSource(
                shader_id,
                1,
                &(shader_src.as_bytes().as_ptr().cast()),
                &(shader_src.len().try_into().unwrap())
            );
            gl::CompileShader(shader_id);

            let mut success: GLint = 0;

            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(INFO_LOG_BUFFER_LEN as usize);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(
                    shader_id,
                    INFO_LOG_BUFFER_LEN,
                    &mut log_len,
                    v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
            }
        }
    }
}

impl Drop for ShaderProgram {

    fn drop(&mut self) {

        unsafe {
            gl::DeleteShader(self.vertex_shader_id);
            gl::DeleteShader(self.fragment_shader_id);
            gl::DeleteProgram(self.program_id);
        }
    }
}
