use gl::types::GLuint;

use crate::texture::{Texture, TexType};



struct Framebuffer {
    id: GLuint,
    width: u32,
    height: u32,
    color_attachment: Texture,
    depth_stencil_attachment: Texture,
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
            width,
            height,
            color_attachment,
            depth_stencil_attachment,
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
}

impl Drop for Framebuffer {

    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.id);
        }
    }
}
