use std::ffi::c_void;

use gl::types::{GLuint, GLint, GLenum};

#[derive(PartialEq)]
pub enum TexType {
    Color,
    DepthStencil,
}

pub struct Texture {
    id: GLuint,
}

impl Texture {

    pub fn new(width: u32, height: u32, texture_type: TexType) -> Self {
        let id: GLuint = 0;

        let internal_format: GLint = gl::RGBA as GLint;
        let format: GLenum = gl::RGBA;
        let data_type: GLenum = gl::UNSIGNED_BYTE;

        if texture_type == TexType::DepthStencil {
            internal_format = gl::DEPTH24_STENCIL8 as GLint;
            format = gl::DEPTH_STENCIL;
            data_type = gl::UNSIGNED_INT_24_8
        }

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                internal_format,
                width as i32,
                height as i32,
                0,
                format,
                data_type,
                0 as *const c_void
            );

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST as GLint
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST as GLint
            );
        }

        Self {
            id,
        }
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Texture {

    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.id);
        }
    }
}
