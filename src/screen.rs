
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
    pos: glm::Vec2,
    quad: Quad,
    back_color: Color,
    framebuffer: Framebuffer,
    shader: ShaderProgram,
}

impl Screen {

    pub fn new(width: u16, height: u16) -> Self {

        let shader = ShaderProgram::new(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);
        Self {
            scale: 1.0,
            pos: glm::vec2(0.0, 0.0),
            quad: Quad::default(),
            back_color: Color {r: 0.1, g: 0.1, b: 0.1, a: 1.0},
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

    }

    pub fn move_canvas(&mut self, pos_delta: &glm::Vec2) {
        self.pos += pos_delta;
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn get_pos(&self) -> glm::Vec2 {
        self.pos
    }

    pub fn render_used_texture(&self) {

        self.framebuffer.bind();

        self.shader.bind();

        self.shader.set_uniform_mat3("transform", &self.compute_transform());

        self.quad.render(&self.shader);

        self.framebuffer.unbind();
    }

    pub fn clear(&self) {
        self.framebuffer.clear(&self.back_color);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.back_color = color;
    }

    pub fn compute_transform(&self) -> glm::Mat3 {

        let translation = glm::translate2d(&glm::Mat3::identity(), &self.pos);

        let scaling = glm::diagonal3x3(&glm::vec3(self.scale, self.scale, 1.0));

        scaling * translation
    }
}

impl Default for Screen {
    fn default() -> Self {

        let shader = ShaderProgram::new(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);

        Self {
            scale: 1.0,
            pos: glm::vec2(0.0, 0.0),
            quad: Quad::default(),
            back_color: Color {r: 0.2, g: 0.2, b: 0.2, a: 1.0},
            framebuffer: Framebuffer::default(),
            shader,
        }
    }
}
