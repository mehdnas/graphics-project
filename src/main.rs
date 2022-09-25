use core::ffi::CStr;
use std::ffi::c_void;

mod common;
mod ui;
mod quad;
mod vao;
mod shader_program;
mod framebuffer;
mod texture;
mod screen;
mod line;

use nalgebra_glm as glm;

use line::Line;
use quad::Quad;
use screen::Screen;
use shader_program::ShaderProgram;

use gl::{self, types::{GLenum, GLuint, GLsizei, GLchar}};
use framebuffer::Framebuffer;
use common::{WINDOW_WIDTH, WINDOW_HEIGHT};

use crate::common::Color;

extern "system" fn gl_debug_proc(
    _source: GLenum,
    _gltype: GLenum,
    _id: GLuint,
    _severity: GLenum,
    _length: GLsizei,
    message: *const GLchar,
    _user_param: *mut c_void
) {
    let message_c_str: &CStr = unsafe {CStr::from_ptr(message)};
    let message_slice: &str = message_c_str.to_str().unwrap();
    eprintln!("GL CALLBACK: {}", message_slice);
}

fn main() {

    let mut gui = ui::Gui::new(
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        "Graficos por Computador",
    );

    unsafe {
        gl::DebugMessageCallback(Some(gl_debug_proc), 0 as *const c_void);
        //gl::Enable(gl::DEBUG_OUTPUT);
    }

    let quad = Quad::default();
    let line_shader_program = ShaderProgram::new(
        "src/shaders/line_vertex.glsl",
        "src/shaders/line_fragment.glsl"
    );
    line_shader_program.bind();
    line_shader_program.set_uniform_vec2(
        "canvas_size",
        &glm::vec2(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)
    );

    let steap_line_shader_program = ShaderProgram::new(
        "src/shaders/line_vertex.glsl",
        "src/shaders/steap_line_fragment.glsl"
    );
    steap_line_shader_program.bind();
    steap_line_shader_program.set_uniform_vec2(
        "canvas_size",
        &glm::vec2(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)
    );

    let framebuffer = Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut screen = Screen::default();

    let mut lines: Vec<Line> = Vec::new();
    let mut line_start: Option<glm::Vec2> = None;

    while !gui.should_close_window() {

        gui.start_frame();

        framebuffer.clear(&Color::default());

        screen.update_zoom(gui.consume_scroll_amount());

        match (gui.consume_cursor_left_press_pos(), line_start) {

            (Some(mut start_pos), None) => {
                transform_pos(&mut start_pos, &screen.get_transform());
                line_start = Some(start_pos);
            }

            (Some(mut end_pos), Some(mut start_pos)) => {
                transform_pos(&mut end_pos, &screen.get_transform());

                if end_pos.x < start_pos.x {
                    let tmp_pos = start_pos;
                    start_pos = end_pos;
                    end_pos = tmp_pos;
                }

                lines.push(Line::new(start_pos, end_pos));
                line_start = None;
            }

            (None, _) => {}
        }

        framebuffer.bind();

        line_shader_program.bind();

        for line in &lines {

            let dx = line.end.x - line.start.x;
            let dy = line.end.y - line.start.y;
            let m;
            let b;
            let shader: &ShaderProgram;


            if dy <= dx {
                m = dy / dx;
                b = line.start.y - m * line.start.x;
                shader = &line_shader_program;
            }
            else {
                m = dx / dy;
                b = line.start.x - m * line.start.y;
                shader = &steap_line_shader_program;
            }

            shader.set_uniform_f32("m", m);
            shader.set_uniform_f32("b", b);
            quad.render(shader);
        }

        framebuffer.unbind();

        screen.render_framebuffer(&framebuffer);

        gui.show(|ui| {
            ui.separator();
            ui.label(" ");
            if ui.button("clear").clicked() {
                line_start = None;
                lines.clear();
            }
            ui.separator();
            ui.label("Lines (start -> end):");
            for line in &lines {
                ui.label(format!(
                    "({}, {}) -> ({}, {})",
                    line.start.x as i32,
                    line.start.y as i32,
                    line.end.x as i32,
                    line.end.y as i32,
                ));
        }

        });

        gui.end_frame();
    }
}

fn transform_pos(pos: &mut glm::Vec2, transform: &glm::Mat3) {
    pos.x = pos.x - WINDOW_WIDTH as f32 / 2_f32;
    pos.y = - pos.y + WINDOW_HEIGHT as f32 / 2_f32;
    let transform_inv = glm::inverse(transform);
    let pos3 = glm::vec3(pos.x, pos.y, 1.0);
    let result = transform_inv * pos3;
    pos.x = result.x;
    pos.y = result.y;
}

fn normalize_pos(pos: &mut glm::Vec2) {
    pos.x = (pos.x * 2_f32) / WINDOW_WIDTH as f32;
    pos.y = (pos.y * 2_f32) / WINDOW_HEIGHT as f32;
}
