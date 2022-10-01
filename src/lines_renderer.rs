
use std::slice::range;

use nalgebra_glm as glm;

use crate::{
    shader_program::{ShaderProgram, self},
    framebuffer::Framebuffer,
    line::Line,
    common::*,
    quad::Quad,
};

pub const CANVAS_WIDTH: u16 = WINDOW_WIDTH as u16;
pub const CANVAS_HEIGHT: u16 = WINDOW_HEIGHT as u16;

pub enum LineAlgorithem {
    SlopeIntercept,
    SlopeInterceptGPU,
}

enum LineKind {
    Moderate,
    Steep,
}

pub struct LinesRenderer {
    line_shader: ShaderProgram,
    steap_line_shader: ShaderProgram,
    canvas: Framebuffer,
    back_color: Color,
    quad: Quad,
}

impl Default for LinesRenderer {
    fn default() -> Self {
        let default_self = Self {
            line_shader: ShaderProgram::new(
                "src/shaders/line_vertex.glsl",
                "src/shaders/line_fragment.glsl"
            ),
            steap_line_shader: ShaderProgram::new(
                "src/shaders/line_vertex.glsl",
                "src/shaders/steap_line_fragment.glsl"
            ),
            canvas: Framebuffer::new(CANVAS_WIDTH, CANVAS_HEIGHT),
            back_color: Color::default(),
            quad: Quad::default(),
        };

        default_self.line_shader.set_uniform_vec2(
            "canvas_size",
            &glm::vec2(CANVAS_WIDTH as f32, CANVAS_HEIGHT as f32)
        );
        default_self.steap_line_shader.set_uniform_vec2(
            "canvas_size",
            &glm::vec2(CANVAS_WIDTH as f32, CANVAS_HEIGHT as f32)
        );

        default_self
    }
}

impl LinesRenderer {

    pub fn render(&self, lines: &Vec<Line>, algorithem: LineAlgorithem) {

        match algorithem {

            LineAlgorithem::SlopeInterceptGPU => {
                self.render_slope_intercept_gpu(lines);
            }

            LineAlgorithem::SlopeIntercept => {
            }
        }
    }

    pub fn render_slope_intercept(&self, lines: &Vec<Line>) {

        for line in lines {

            let (m, b, line_kind) = LinesRenderer::comput_m_b(line);
            let (tex_width, tex_height) = self.canvas.get_size();
            let mut texture = vec![ColorU8::default(); (tex_width * tex_height) as usize];

            let line_pixels;

            match line_kind {

                LineKind::Moderate => {
                    let pixels_count = (line.end.x - line.start.x + 1.0) as usize;

                    line_pixels = vec![glm::U16Vec2::from_element(0); pixels_count];

                    for i in 0..line_pixels.len() {
                        line_pixels[i] = glm::U16Vec2::new(
                            (i as f32 + tex_width as f32 / 2.0) as u16,
                            (-(m * i as f32 + b) + tex_height as f32 / 2.0).round() as u16,
                        );
                    }
                }

                LineKind::Steep => {

                    let pixels_count = (line.end.y - line.end.y + 1.0) as usize;

                    line_pixels = vec![glm::U16Vec2::from_element(0); pixels_count];

                    for i in 0..line_pixels.len() {
                        line_pixels[i] = glm::U16Vec2::new(
                            (m * i as f32 + b + tex_width as f32 / 2.0).round() as u16,
                            (-(i as f32) + tex_height as f32 / 2.0) as u16,
                        );
                    }
                }
            }

            for pixel_pos in &line_pixels {
                texture[(pixel_pos.x * tex_width + pixel_pos.y) as usize] = ColorU8{
                    r: 1, g: 1, b: 1, a: 1
                };
            }

            self.canvas.set_color_data(&texture);
        }
    }

    pub fn render_slope_intercept_gpu(&self, lines: &Vec<Line>) {

        self.canvas.clear(&self.back_color);
        self.canvas.bind();

        for line in lines {

            let (m, b, line_kind) = LinesRenderer::comput_m_b(line);
            let shader;

            match line_kind {

                LineKind::Moderate => {
                    shader = &self.line_shader;
                }

                LineKind::Steep => {
                    shader = &self.steap_line_shader;
                }
            }

            shader.set_uniform_f32("m", m);
            shader.set_uniform_f32("b", b);
            self.quad.render(shader);
        }

        self.canvas.unbind();
    }

    pub fn use_canvas_color_attachment(&self) {
        self.canvas.use_color_attachment()
    }

    fn comput_m_b(line: &Line) -> (f32, f32, LineKind) {

        let dx = line.end.x - line.start.x;
        let dy = line.end.y - line.start.y;
        let m;
        let b;
        let line_kind;

        if dy.abs() <= dx.abs() {
            m = dy / dx;
            b = line.start.y - m * line.start.x;
            line_kind = LineKind::Moderate;
        }
        else {
            m = dx / dy;
            b = line.start.x - m * line.start.y;
            line_kind = LineKind::Steep;
        }

        (m, b, line_kind)
    }
}
