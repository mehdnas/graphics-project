use gl::types::*;

use crate::{texture::{Texture, TexType}, common::{WINDOW_WIDTH, WINDOW_HEIGHT, Color, ColorU8}};


pub struct Framebuffer {
    id: GLuint,
    width: u16,
    height: u16,
    color_attachment: Option<Texture>,
}

impl Default for Framebuffer {
    fn default() -> Self {
        Self {
            id: 0,
            width: WINDOW_WIDTH as u16,
            height: WINDOW_HEIGHT as u16,
            color_attachment: None
        }
    }
}

impl Framebuffer {

    pub fn new(width: u16, height: u16) -> Self {

        let mut id: GLuint = 0;
        let color_attachment = Texture::new_blank(width, height, TexType::Color);
        let depth_stencil_attachment = Texture::new_blank(
            width, height, TexType::DepthStencil
        );

        unsafe {
            gl::GenFramebuffers(1, &mut id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, id);

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                color_attachment.get_id(),
                0
            );
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::TEXTURE_2D,
                depth_stencil_attachment.get_id(),
                0
            );

            assert_eq!(
                gl::CheckFramebufferStatus(gl::FRAMEBUFFER),
                gl::FRAMEBUFFER_COMPLETE
            );

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Self {
            id,
            width,
            height,
            color_attachment: Some(color_attachment),
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
            gl::Viewport(0, 0, self.width as i32, self.height as i32);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn clear(&self, color: &Color) {
        self.bind();
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn use_color_attachment(&self) {

        match &self.color_attachment {
            Some(texture) => {
                texture.bind()
            }
            None => {
                panic!("Probably trying to use default framebuffer's color attachment");
            }
        }
    }

    pub fn set_color_data(&mut self, color_data: &[ColorU8]) {
        match &mut self.color_attachment {
            Some(texture) => {
                texture.set_data(color_data);
            }

            None => {}
        }
    }
}

impl Drop for Framebuffer {

    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.id);
        }
    }
}
