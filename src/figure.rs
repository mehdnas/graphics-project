
use nalgebra_glm as glm;

use crate::{
    quad::Quad,
    texture::Texture,
    shader_program::ShaderProgram,
};

const VERTEX_SHADER_SRC: &str = "src/shaders/fig_vertex.glsl";
const FRAGMENT_SHADER_SRC: &str = "src/shaders/fig_fragment.glsl";
const TRANSFORM_UNIFORM_NAME: &str = "transform";

pub struct Figure {
    quad: Quad,
    shader: ShaderProgram,
    scale: f32,
    shearing: glm::Vec2,
    rotation: f32,
    position: glm::Vec2,
}

impl Default for Figure {
    fn default() -> Self {
        Self {
            quad: Quad::default(),
            shader: ShaderProgram::new(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC),
            scale: 1.0,
            shearing: glm::vec2(0.0, 0.0),
            rotation: 0.0,
            position: glm::vec2(0.0, 0.0),
        }
    }
}

impl Figure {

    pub fn get_position(&self) -> glm::Vec2 {
        self.position.clone()
    }

    pub fn set_position(&mut self, new_position: &glm::Vec2) {
        self.position = *new_position;
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn set_scale(&mut self, new_scale: f32) {
        self.scale = new_scale;
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, new_rotation: f32) {
        self.rotation = new_rotation;
    }

    pub fn get_shearing(&self) -> glm::Vec2 {
        self.shearing.clone()
    }

    pub fn set_shearing(&mut self, new_shrearing: &glm::Vec2) {
        self.shearing = *new_shrearing;
    }

    pub fn render(&self, texture: &Texture) {
        texture.bind();
        let mut transform = glm::diagonal3x3(&glm::vec3(self.scale, self.scale, 1.0));
        transform = glm::shear2d_x(&transform, self.shearing.x);
        transform = glm::shear2d_y(&transform, self.shearing.y);
        transform = glm::rotate2d(&transform, self.rotation);
        transform = glm::translate2d(&transform, &self.position);
        self.shader.set_uniform_mat3(TRANSFORM_UNIFORM_NAME, &transform);
        self.quad.render(&self.shader);
    }
}
