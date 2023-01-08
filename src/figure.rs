
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
    texture: Texture,
    base_transform: glm::Mat3,
    scale: glm::Vec2,
    shearing: glm::Vec2,
    rotation: f32,
    position: glm::Vec2,
}

impl Default for Figure {
    fn default() -> Self {
        Self {
            quad: Quad::default(),
            shader: ShaderProgram::new(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC),
            texture: Texture::default(),
            base_transform: glm::identity(),
            scale: glm::vec2(1.0, 1.0),
            shearing: glm::vec2(0.0, 0.0),
            rotation: 0.0,
            position: glm::vec2(0.0, 0.0),
        }
    }
}

impl Figure {

    pub fn new(texture: Texture) -> Self {

        let base_transform = glm::diagonal3x3(&glm::vec3(
            1.0, //texture.get_width() as f32 / texture.get_height() as f32,
            1.0,
            1.0
        ));

        Self {
            texture,
            base_transform,
            ..Default::default()
        }
    }

    pub fn get_position(&self) -> glm::Vec2 {
        self.position.clone()
    }

    pub fn set_position(&mut self, new_position: &glm::Vec2) {
        self.position = *new_position;
    }

    pub fn get_scale(&self) -> glm::Vec2 {
        self.scale
    }

    pub fn set_scale(&mut self, new_scale: glm::Vec2) {
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

    pub fn render(&self) {

        self.texture.bind();

        let mut transform = glm::identity();
        transform = glm::translate2d(&transform, &self.position);
        transform = glm::rotate2d(&transform, self.rotation);
        transform = glm::shear2d_x(&transform, self.shearing.x);
        transform = glm::shear2d_y(&transform, self.shearing.y);
        transform = glm::scale2d(&transform, &self.scale);
        transform = transform * self.base_transform;

        self.shader.set_uniform_mat3(TRANSFORM_UNIFORM_NAME, &transform);
        self.quad.render(&self.shader);
    }
}
