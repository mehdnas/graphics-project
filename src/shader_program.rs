use std::{fs, convert::TryInto};

use gl::types::{GLuint, GLint, GLsizei};
use nalgebra_glm as glm;

const INFO_LOG_BUFFER_LEN: GLsizei = 1024;


pub struct ShaderProgram {
    vertex_shader_id: GLuint,
    fragment_shader_id: GLuint,
    program_id: GLuint,
}

impl ShaderProgram {

    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> Self {

        let vertex_shader_id: GLuint;
        let fragment_shader_id: GLuint;
        let program_id: GLuint;

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

    pub fn set_uniform_f32(&self, name: &str, float: f32) {

        let cname = std::ffi::CString::new(name).expect("CString new failed");

        self.bind();

        unsafe {
            gl::Uniform1f(
                self.get_uniform_location(name),
                float
            );
        }
    }

    pub fn set_uniform_mat3(&self, name: &str, mat: &glm::Mat3) {

        self.bind();

        unsafe {
            gl::UniformMatrix3fv(
                self.get_uniform_location(name),
                1,
                gl::FALSE,
                glm::value_ptr(&mat).as_ptr().cast()
            );
        }
    }

    pub fn set_uniform_vec2(&self, name: &str, vec: &glm::Vec2) {

        self.bind();

        unsafe {
            gl::Uniform2fv(
                self.get_uniform_location(name),
                1,
                glm::value_ptr(&vec).as_ptr().cast()
            );
        }
    }

    pub fn set_uniform_vec3(&self, name: &str, vec: &glm::Vec3) {

        self.bind();

        unsafe {
            gl::Uniform2fv(
                self.get_uniform_location(name),
                1,
                glm::value_ptr(&vec).as_ptr().cast()
            );
        }
    }

    fn get_uniform_location(&self, name: &str) -> GLint {

        let location: GLint;
        let cname = std::ffi::CString::new(name).expect("CString new failed");

        unsafe {
            location = gl::GetUniformLocation(
                self.program_id,
                cname.to_bytes().as_ptr().cast()
            );
            assert_ne!(location, -1, "Failed to get uniform location");
        }

        location
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
                panic!(
                    "Vertex Compile Error: {} \n {}",
                    String::from_utf8_lossy(&v),
                    shader_src
                );
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
