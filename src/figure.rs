
use nalgebra_glm as glm;

use crate::{
    quad::Quad,
    shader_program::ShaderProgram,
};

const VERTEX_SHADER_SRC: &str = "src/shaders/fig_vertex.glsl";
const FRAGMENT_SHADER_SRC: &str = "src/shaders/fig_fragment.glsl";
const TRANSFORM_UNIFORM_NAME: &str = "transform";
const ITERATIONS_UNIFORM_NAME: &str = "iteration_count";
const COLOR_JUMP_UNIFORM_NAME: &str = "color_jump";

pub struct Figure {
    quad: Quad,
    shader: ShaderProgram,
    base_transform: glm::Mat3,
    scale: glm::Vec2,
    shearing: glm::Vec2,
    rotation: f32,
    position: glm::Vec2,
    iterations: u32,
    color_jump: u32,
}

impl Default for Figure {
    fn default() -> Self {
        Self {
            quad: Quad::default(),
            shader: ShaderProgram::new(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC),
            base_transform: glm::identity(),
            scale: glm::vec2(1.0, 1.0),
            shearing: glm::vec2(0.0, 0.0),
            rotation: 0.0,
            position: glm::vec2(0.0, 0.0),
            iterations: 1000,
            color_jump: 100,
        }
    }
}

impl Figure {

    pub fn new() -> Self {

        let quad = Quad::from_vertex_positions(
            [(-2., 1.), (1., 1.), (1., -1.), (-2., -1.)]
        );

        let base_transform = glm::diagonal3x3(&glm::vec3(1.0, 1.0, 1.0));

        Self {
            quad,
            base_transform,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn get_position(&self) -> glm::Vec2 {
        self.position.clone()
    }

    #[allow(dead_code)]
    pub fn set_position(&mut self, new_position: &glm::Vec2) {
        self.position = *new_position;
    }

    #[allow(dead_code)]
    pub fn get_scale(&self) -> glm::Vec2 {
        self.scale
    }

    #[allow(dead_code)]
    pub fn set_scale(&mut self, new_scale: glm::Vec2) {
        self.scale = new_scale;
    }

    #[allow(dead_code)]
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    #[allow(dead_code)]
    pub fn set_rotation(&mut self, new_rotation: f32) {
        self.rotation = new_rotation;
    }

    #[allow(dead_code)]
    pub fn get_shearing(&self) -> glm::Vec2 {
        self.shearing.clone()
    }

    #[allow(dead_code)]
    pub fn set_shearing(&mut self, new_shrearing: &glm::Vec2) {
        self.shearing = *new_shrearing;
    }

    #[allow(dead_code)]
    pub fn get_iterations(&self) -> u32 {
        self.iterations
    }

    #[allow(dead_code)]
    pub fn set_iterations(&mut self, new_iterations: u32) {
        self.iterations = new_iterations;
    }

    #[allow(dead_code)]
    pub fn get_color_jump(&self) -> u32 {
        self.color_jump
    }

    #[allow(dead_code)]
    pub fn set_color_jump(&mut self, new_value: u32) {
        self.color_jump = new_value;
    }

    pub fn render(&self) {

        let mut transform = glm::identity();
        transform = glm::scale2d(&transform, &self.scale);
        transform = glm::translate2d(&transform, &self.position);
        transform = glm::rotate2d(&transform, self.rotation);
        transform = glm::shear2d_x(&transform, self.shearing.x);
        transform = glm::shear2d_y(&transform, self.shearing.y);
        transform = transform * self.base_transform;

        self.shader.set_uniform_mat3(TRANSFORM_UNIFORM_NAME, &transform);
        self.shader.set_uniform_u32(ITERATIONS_UNIFORM_NAME, self.iterations);
        self.shader.set_uniform_u32(COLOR_JUMP_UNIFORM_NAME, self.color_jump);
        self.quad.render(&self.shader);
    }
}
