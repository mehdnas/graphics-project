
use nalgebra_glm as glm;

use crate::{
    framebuffer::Framebuffer,
    quad::Quad,
    shader_program::ShaderProgram, common::Color
};

const ZOOM_DELTA :f32 = 0.1_f32;
const MAX_SCALE: f32 = 20.0_f32;
const MIN_SCALE: f32 = 0.01_f32;
const VERTEX_SHADER_SRC: &str = "src/shaders/screen_vertex.glsl";
const FRAGMENT_SHADER_SRC: &str = "src/shaders/screen_fragment.glsl";


pub struct Screen {
    scale: f32,
    transform: glm::Mat3,
    quad: Quad,
    back_color: Color,
    framebuffer: Framebuffer,
    shader: ShaderProgram,
}

impl Screen {

    pub fn new(width: u32, height: u32) -> Self {

        let shader = ShaderProgram::new(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);
        Self {
            scale: 1.0,
            quad: Quad::default(),
            transform: glm::diagonal3x3(&glm::vec3(1.0, 1.0, 1.0)),
            back_color: Color::default(),
            framebuffer: Framebuffer::new(width, height),
            shader,
        }
    }

    pub fn update_zoom(&mut self, scroll_amount: f64) {

        self.scale += scroll_amount as f32 * ZOOM_DELTA;

        if self.scale > MAX_SCALE {
            self.scale = MAX_SCALE;
        }
        else if self.scale < MIN_SCALE {
            self.scale = MIN_SCALE
        }

        self.transform = glm::diagonal3x3(&glm::vec3(self.scale, self.scale, 1.0));
    }

    pub fn get_transform(&self) -> glm::Mat3 {
        self.transform
    }

    pub fn render_framebuffer(&self, framebuffer: &Framebuffer) {

        self.framebuffer.bind();

        self.shader.bind();

        let transform = glm::diagonal3x3(&glm::vec3(self.scale, self.scale, 1.0));

        self.shader.set_uniform_mat3("transform", &transform);

        framebuffer.use_color_attachment();

        self.quad.render(&self.shader);

        self.framebuffer.unbind();
    }

    pub fn clear(&self) {
        self.framebuffer.clear(&self.back_color);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.back_color = color;
    }
}

impl Default for Screen {
    fn default() -> Self {

        let shader = ShaderProgram::new(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);

        Self {
            scale: 1.0,
            transform: glm::diagonal3x3(&glm::vec3(1.0, 1.0, 1.0)),
            quad: Quad::default(),
            back_color: Color::default(),
            framebuffer: Framebuffer::default(),
            shader,
        }
    }
}
