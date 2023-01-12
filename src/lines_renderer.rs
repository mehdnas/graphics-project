
use nalgebra_glm as glm;

use crate::{
    shader_program::ShaderProgram,
    framebuffer::Framebuffer,
    line::Line,
    common::*,
    quad::Quad,
};

pub const CANVAS_WIDTH: u16 = WINDOW_WIDTH as u16;
pub const CANVAS_HEIGHT: u16 = WINDOW_HEIGHT as u16;

pub enum LineAlgorithem {
    SlopeIntercept,
    SlopeInterceptFS,
    DDA,
    Bresenham,
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
                Some((
                    "src/shaders/line_vertex.glsl",
                    "src/shaders/line_fragment.glsl"
                )),
                None,
            ),
            steap_line_shader: ShaderProgram::new(
                Some((
                    "src/shaders/line_vertex.glsl",
                    "src/shaders/steap_line_fragment.glsl"
                )),
                None,
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

    pub fn render(&mut self, lines: &Vec<Line>, algorithem: &LineAlgorithem) {

        match algorithem {

            LineAlgorithem::SlopeInterceptFS => {
                self.render_slope_intercept_gpu(lines);
            }

            LineAlgorithem::SlopeIntercept => {
                self.render_on_cpu(lines, Self::render_slope_intercept);
            }

            LineAlgorithem::DDA => {
                self.render_on_cpu(lines, Self::render_dda);
            }

            LineAlgorithem::Bresenham => {
                self.render_on_cpu(lines, Self::render_bresenham);
            }
        }
    }

    pub fn render_on_cpu(
        &mut self,
        lines: &Vec<Line>,
        line_render_fn: fn(&Line, u16, u16) -> Vec<glm::U16Vec2>
    ) {

        let (tex_width, tex_height) = self.canvas.get_size();
        let size = (tex_width as usize * tex_height as usize) as usize;
        let mut texture = vec![ColorU8::default(); size];

        for line in lines {

            let line_pixels = line_render_fn(&line, tex_width, tex_height);

            for pixel_pos in &line_pixels {

                let index: usize = (
                      pixel_pos.y as usize
                    * tex_width as usize
                    + pixel_pos.x as usize
                ) as usize;

                if index < texture.len() {
                    texture[index] = ColorU8{r:255, g: 255, b: 255, a:255};
                }
            }

        }
        self.canvas.set_color_data(&texture);
    }

    pub fn render_bresenham(
        line: &Line,
        tex_width: u16,
        tex_height: u16,
    ) -> Vec<glm::U16Vec2> {

        let xi = line.start.x.round() as i32;
        let yi = line.start.y.round() as i32;
        let xf = line.end.x.round() as i32;
        let yf = line.end.y.round() as i32;

        if (yf - yi).abs() < (xf - xi).abs() {
            if xi > xf {
                Self::bresenham_low(xf, yf, xi, yi, tex_width, tex_height)
            } else {
                Self::bresenham_low(xi, yi, xf, yf, tex_width, tex_height)
            }
        } else {
            if yi > yf {
                Self::bresenham_high(xf, yf, xi, yi, tex_width, tex_height)
            } else {
                Self::bresenham_high(xi, yi, xf, yf, tex_width, tex_height)
            }
        }

    }

    fn bresenham_low(
        xi: i32, yi: i32,
        xf: i32, yf: i32,
        tex_width: u16,
        tex_height: u16,
    ) -> Vec<glm::U16Vec2> {

        let mut line_pixels = vec![
            glm::U16Vec2::from_element(0);
            (xf - xi + 1) as usize
        ];

        let dx = xf - xi;
        let dy = yf - yi;
        let (dy, ys) = if dy < 0 {(-dy, -1)} else {(dy, 1)};

        let mut e = (2 * dy) - dx;

        let mut y = yi;
        let mut x = xi;

        for i in 0..line_pixels.len() {

            let tex_x = (x + tex_width as i32 / 2) as u16;
            let tex_y = (y + tex_height as i32 / 2) as u16;
            line_pixels[i] = glm::U16Vec2::new(tex_x, tex_y);

            if e > 0 {
                y = y + ys;
                e = e + (2 * (dy - dx));
            } else {
                e = e + 2 * dy;
            }
            x += 1;
        }

        line_pixels
    }

    fn bresenham_high(
        xi: i32, yi: i32,
        xf: i32, yf: i32,
        tex_width: u16,
        tex_height: u16,
    ) -> Vec<glm::U16Vec2> {

        let mut line_pixels = vec![
            glm::U16Vec2::from_element(0);
            (yf - yi + 1) as usize
        ];

        let dx = xf - xi;
        let dy = yf - yi;
        let (dx, xs) = if dx < 0 {(-dx, -1)} else {(dx, 1)};

        let mut e = (2 * dx) - dy;

        let mut y = yi;
        let mut x = xi;

        for i in 0..line_pixels.len() {

            let tex_x = (x + tex_width as i32 / 2) as u16;
            let tex_y = (y + tex_height as i32 / 2) as u16;
            line_pixels[i] = glm::U16Vec2::new(tex_x, tex_y);

            if e > 0 {
                x = x + xs;
                e = e + (2 * (dx - dy));
            } else {
                e = e + 2 * dx;
            }
            y += 1;
        }

        line_pixels
    }

    pub fn render_dda(
        line: &Line,
        tex_width: u16,
        tex_height: u16,
    ) -> Vec<glm::U16Vec2> {

        let d_pos = line.end - line.start;
        let dx = d_pos.x;
        let dy = d_pos.y;
        let m;

        if dx.abs() >= dy.abs() {
            m = dx.abs();
        } else {
            m = dy.abs();
        }

        let dx = dx / m;
        let dy = dy / m;

        let mut line_pixels = vec![glm::U16Vec2::from_element(0); m.round() as usize];

        let mut x = line.start.x;
        let mut y = line.start.y;

        for i in 0..line_pixels.len() {

            let tex_x = (x + tex_width as f32 / 2.0).round() as u16;
            let tex_y = (y + tex_height as f32 / 2.0).round() as u16;

            line_pixels[i] = glm::U16Vec2::new(tex_x, tex_y);

            x += dx;
            y += dy;
        }

        line_pixels
    }

    pub fn render_slope_intercept(
        line: &Line,
        tex_width: u16,
        tex_height: u16,
    ) -> Vec<glm::U16Vec2> {

        let (m, b, line_kind) = LinesRenderer::comput_m_b(line);
        let mut line_pixels;

        match line_kind {

            LineKind::Moderate => {

                let (start, end) = LinesRenderer::x_order_line_ends(line);
                let pixels_count = (end.x - start.x + 1.0) as usize;

                line_pixels = vec![glm::U16Vec2::from_element(0); pixels_count];

                for i in 0..line_pixels.len() {

                    let x = i as f32 + start.x;
                    let y = ((m * x + b) + tex_height as f32 / 2.0).round() as u16;
                    let x = (x + tex_width as f32 / 2.0) as u16;

                    line_pixels[i] = glm::U16Vec2::new(x,y);
                }
            }

            LineKind::Steep => {

                let (start, end) = LinesRenderer::y_order_line_ends(line);
                let pixels_count = (end.y - start.y + 1.0).abs() as usize;

                line_pixels = vec![glm::U16Vec2::from_element(0); pixels_count];

                for i in 0..line_pixels.len() {

                    let y = i as f32 + start.y;
                    let x = ((m * y + b) + tex_width as f32 / 2.0).round() as u16;
                    let y = (y + tex_height as f32 / 2.0) as u16;

                    line_pixels[i] = glm::U16Vec2::new(x, y);
                }
            }
        }

        line_pixels
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

    fn x_order_line_ends(line: &Line) -> (&glm::Vec2, &glm::Vec2) {
        if line.start.x > line.end.x {
            (&line.end, &line.start)
        }
        else {
            (&line.start, &line.end)
        }
    }

    fn y_order_line_ends(line: &Line) -> (&glm::Vec2, &glm::Vec2) {
        if line.start.y > line.end.y {
            (&line.end, &line.start)
        }
        else {
            (&line.start, &line.end)
        }
    }
}
