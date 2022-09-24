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
    let noise_shader_program = ShaderProgram::new(
        "src/shaders/noise_vertex.glsl",
        "src/shaders/noise_fragment.glsl"
    );
    let framebuffer = Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut screen = Screen::default();

    let mut lines: Vec<Line> = Vec::new();
    let mut line_start: Option<glm::Vec2> = None;

    while !gui.should_close_window() {

        gui.start_frame();

        screen.update_zoom(gui.consume_scroll_amount());

        match (gui.consume_cursor_left_press_pos(), line_start) {

            (Some(mut start_pos), None) => {
                transform_pos(&mut start_pos);
                line_start = Some(start_pos);
            }

            (Some(mut end_pos), Some(_)) => {
                transform_pos(&mut end_pos);
                lines.push(Line::new(
                    line_start.unwrap(),
                    end_pos
                ));
                line_start = None;
            }

            (None, _) => {}
        }

        framebuffer.bind();

        quad.render(&noise_shader_program);

        framebuffer.unbind();

        screen.render_framebuffer(&framebuffer);

        println!("Lines {{");

        for line in &lines {
            println!(
                "({}, {}) -> ({}, {})",
                line.start.x,
                line.start.y,
                line.end.x,
                line.end.y,
            );
        }

        println!("}}");

        gui.show(|ui| {
            ui.separator();
            ui.label(" ");
            if ui.button("clear").clicked() {
                lines.clear();
            }
        });

        gui.end_frame();
    }
}

fn transform_pos(pos: &mut glm::Vec2) {
    pos.x = pos.x - WINDOW_WIDTH as f32 / 2_f32;
    pos.y = - pos.y + WINDOW_HEIGHT as f32 / 2_f32;
}

fn normalize_pos(pos: &mut glm::Vec2) {
    pos.x = (pos.x * 2_f32) / WINDOW_WIDTH as f32;
    pos.y = (pos.y * 2_f32) / WINDOW_HEIGHT as f32;
}
