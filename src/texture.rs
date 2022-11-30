use std::ffi::c_void;

use gl::types::{GLuint, GLint, GLenum};

use crate::common::ColorU8;

#[derive(PartialEq)]
pub enum TexType {
    Color,
    DepthStencil,
}

pub struct Texture {
    id: GLuint,
    width: u16,
    height: u16,
    tex_type: TexType
}

impl Default for Texture {
    fn default() -> Self {
        Self::new(16, 16, TexType::Color, None)
    }
}

impl Texture {

    pub fn new(
        width: u16, height: u16, tex_type: TexType,
        pixels: Option<&Vec<(u8, u8, u8, u8)>>
    ) -> Self {

        let mut id: GLuint = 0;
        let (internal_format, format, data_type) = Texture::get_formats(&tex_type);

        unsafe {

            let pixel_ptr = match pixels {
                Some(data) =>data.as_ptr() as *const c_void,
                None => 0 as *const c_void,
            };

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
                pixel_ptr,
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
            width,
            height,
            tex_type,
        }
    }

    pub fn get_width(&self) -> u16 {
        return self.width;
    }

    pub fn get_height(&self) -> u16 {
        return self.height;
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(
                gl::TEXTURE_2D,
                self.id
            );
        }
    }

    pub fn set_data(&mut self, color_data: &[ColorU8]) {

        let (internal_format, format, data_type) = Texture::get_formats(&self.tex_type);

        self.bind();

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                internal_format,
                self.width as i32,
                self.height as i32,
                0,
                format,
                data_type,
                color_data.as_ptr().cast()
            );
        }
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    fn get_formats(tex_type: &TexType) -> (GLint, GLenum, GLenum) {

        let mut internal_format: GLint = gl::RGBA as GLint;
        let mut format: GLenum = gl::RGBA;
        let mut data_type: GLenum = gl::UNSIGNED_BYTE;

        if matches!(tex_type, TexType::DepthStencil) {
            internal_format = gl::DEPTH24_STENCIL8 as GLint;
            format = gl::DEPTH_STENCIL;
            data_type = gl::UNSIGNED_INT_24_8;
        }

        (internal_format, format, data_type)
    }
}

impl Drop for Texture {

    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.id);
        }
    }
}
