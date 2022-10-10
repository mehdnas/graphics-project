use std::{fs, convert::TryInto};

use gl::types::{GLuint, GLint, GLsizei, GLenum};
use nalgebra_glm as glm;

const INFO_LOG_BUFFER_LEN: GLsizei = 1024;


#[allow(dead_code)]
pub struct ShaderProgram {
    vertex_shader_id: Option<GLuint>,
    fragment_shader_id: Option<GLuint>,
    compute_shader_id: Option<GLuint>,
    program_id: GLuint,
}

impl ShaderProgram {

    pub fn new(
        graphics_shaders_paths: Option<(&str, &str)>,
        compute_shader_path: Option<&str>,
    ) -> Self {

        let vertex_shader_id: Option<GLuint>;
        let fragment_shader_id: Option<GLuint>;
        let compute_shader_id: Option<GLuint>;
        let program_id: GLuint;

        if matches!(graphics_shaders_paths, None) && matches!(compute_shader_path, None) {
            panic!("Neither graphics shaders nor compute shader paths were given");
        }

        unsafe {
            program_id = gl::CreateProgram();
        }

        match graphics_shaders_paths {

            Some((vs_path, fs_path)) => {

                let vs_id = Self::create_shader(vs_path, gl::VERTEX_SHADER);
                let fs_id = Self::create_shader(fs_path, gl::FRAGMENT_SHADER);

                unsafe {
                    gl::AttachShader(program_id, vs_id);
                    gl::AttachShader(program_id, fs_id);
                }

                vertex_shader_id = Some(vs_id);
                fragment_shader_id = Some(fs_id);
            }

            None => {
                vertex_shader_id = None;
                fragment_shader_id = None;
            }

        }

        match compute_shader_path {

            Some(path) => {

                let cs_id = Self::create_shader(path, gl::COMPUTE_SHADER);

                unsafe {
                    gl::AttachShader(program_id, cs_id);
                }

                compute_shader_id = Some(cs_id);
            }

            None => {
                compute_shader_id = None;
            }
        };

        unsafe {
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
            compute_shader_id,
            program_id,
        }
    }

    fn create_shader(src_path: &str, shader_type: GLenum) -> GLuint {

        let shader_id: GLuint;

        let shader_src = fs::read_to_string(src_path)
            .expect("Could not read the compute shade file");

        unsafe {
            shader_id = gl::CreateShader(shader_type);
        }
        assert_ne!(shader_id, 0);

        Self::compile_shader(shader_id, &shader_src);

        shader_id
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    pub fn set_uniform_f32(&self, name: &str, float: f32) {

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

    #[allow(dead_code)]
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
            match self.vertex_shader_id {

                Some(vs_id) => gl::DeleteShader(vs_id),

                None => {}
            }

            match self.fragment_shader_id {

                Some(fs_id) => gl::DeleteShader(fs_id),

                None => {}
            }

            gl::DeleteProgram(self.program_id);
        }
    }
}
