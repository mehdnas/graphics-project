use gl::types::{GLuint, GLint};

use crate::{texture::{Texture, TexType}, common::{WINDOW_WIDTH, WINDOW_HEIGHT}};


pub struct Framebuffer {
    id: GLuint,
    width: GLint,
    height: GLint,
    color_attachment: Option<Texture>,
}

impl Default for Framebuffer {
    fn default() -> Self {
        Self {
            id: 0,
            width: WINDOW_WIDTH as GLint,
            height: WINDOW_HEIGHT as GLint,
            color_attachment: None
        }
    }
}

impl Framebuffer {

    pub fn new(width: u32, height: u32) -> Self {

        let mut id: GLuint = 0;
        let color_attachment = Texture::new(width, height, TexType::Color);
        let depth_stencil_attachment = Texture::new(
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
            width: width as GLint,
            height: height as GLint,
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

    pub fn use_color_attachment(&self) {

        match &self.color_attachment {
            Some(texture) => {
                unsafe {
                    gl::BindTexture(
                        gl::TEXTURE_2D,
                        texture.get_id()
                    );
                }
            }
            None => {
                panic!("Probably trying to use default framebuffer's color attachment");
            }
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
