use core::ffi::CStr;
use std::{ffi::c_void, time::{Instant, Duration}};

mod common;
mod ui;
mod quad;
mod vao;
mod shader_program;
mod framebuffer;
mod texture;
mod screen;
mod line;
mod lines_renderer;

use nalgebra_glm as glm;
use egui_glfw_gl as egui_backend;
use egui_backend::glfw;

use line::Line;
use screen::Screen;
use lines_renderer::{LinesRenderer, LineAlgorithem};

use gl::{self, types::{GLenum, GLuint, GLsizei, GLchar}};
use common::{WINDOW_WIDTH, WINDOW_HEIGHT};
use lines_renderer::{CANVAS_HEIGHT, CANVAS_WIDTH};
use ui::Gui;

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

    let move_speed = glm::Vec2::new(
        CANVAS_WIDTH as f32 / 500.0, CANVAS_HEIGHT as f32 / 500.0
    );

    let mut screen = Screen::default();

    let mut lines_renderer = LinesRenderer::default();
    let mut line_algorithem = LineAlgorithem::SlopeIntercept;

    let mut lines: Vec<Line> = Vec::new();
    let mut line_start: Option<glm::Vec2> = None;

    let mut start = Instant::now();
    let mut dt = Duration::from_secs_f32(1.0 / 60.0);

    while !gui.should_close_window() {

        gui.start_frame();

        screen.update_zoom(gui.consume_scroll_amount());

        screen.move_canvas(&get_move_deltas(&gui, &dt, &move_speed));

        match (gui.consume_cursor_left_press_pos(), line_start) {

            (Some(mut start_pos), None) => {
                transform_pos(&mut start_pos, &screen);
                line_start = Some(start_pos);
            }

            (Some(mut end_pos), Some(start_pos)) => {
                transform_pos(&mut end_pos, &screen);

                lines.push(Line::new(start_pos, end_pos));
                line_start = None;
            }

            (None, _) => {}
        }

        lines_renderer.render(&lines, &line_algorithem);

        screen.clear();

        lines_renderer.use_canvas_color_attachment();

        screen.render_used_texture();

        render_gui(&gui, &screen, &mut line_algorithem, &mut lines, &mut line_start);

        gui.end_frame();

        dt = start.elapsed();
        start = Instant::now();
    }
}

fn render_gui(
    gui: &Gui,
    screen: &Screen,
    line_algorithem: &mut LineAlgorithem,
    lines: &mut Vec<Line>,
    line_start: &mut Option<glm::Vec2>
) {

        gui.show(|ui| {

            ui.separator();

            if ui.radio(
                matches!(line_algorithem, LineAlgorithem::SlopeIntercept),
                "Slope intercept"
            ).clicked() {
                *line_algorithem = LineAlgorithem::SlopeIntercept;
            }

            if ui.radio(
                matches!(line_algorithem, LineAlgorithem::SlopeInterceptFS),
                "Slope intercept Fragment shader"
            ).clicked() {
                *line_algorithem = LineAlgorithem::SlopeInterceptFS;
            }

            if ui.radio(
                matches!(line_algorithem, LineAlgorithem::DDA),
                "DDA"
            ).clicked() {
                *line_algorithem = LineAlgorithem::DDA;
            }

            if ui.radio(
                matches!(line_algorithem, LineAlgorithem::Bresenham),
                "Bresenham"
            ).clicked() {
                *line_algorithem = LineAlgorithem::Bresenham;
            }

            ui.separator();

            let mut cursor_pos = gui.get_cursor_pos();
            transform_pos(&mut cursor_pos, screen);

            ui.label(format!(
                "Cursor position: ({}, {})",
                cursor_pos.x as i32, cursor_pos.y as i32
            ));

            ui.separator();

            ui.label("Lines (start -> end):");
            if ui.button("clear").clicked() {
                *line_start = None;
                lines.clear();
            }

            for line in lines {
                ui.label(format!(
                    "({}, {}) -> ({}, {})",
                    line.start.x as i32,
                    line.start.y as i32,
                    line.end.x as i32,
                    line.end.y as i32,
                ));

            }
            match line_start {
                Some(pos) => {
                    ui.label(format!("({}, {}) -> ", pos.x as i32, pos.y as i32));
                }
                None => {}
            }
        });

}

fn transform_pos(pos: &mut glm::Vec2, screen: &Screen) {
    pos.x = pos.x - CANVAS_WIDTH as f32 / 2_f32;
    pos.y = - pos.y + CANVAS_HEIGHT as f32 / 2_f32;
    let scale = screen.get_scale();
    let translationv = screen.get_pos().component_mul(
        &glm::vec2(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0)
    );
    let scale_mat = glm::diagonal3x3(&glm::vec3(scale, scale, 1.0));
    let translation_mat = glm::translate2d(&glm::Mat3::identity(), &translationv);
    let transform_inv = glm::inverse(&(scale_mat * translation_mat));
    let pos3 = glm::vec3(pos.x, pos.y, 1.0);
    let result = transform_inv * pos3;
    pos.x = result.x;
    pos.y = result.y;
}

#[allow(dead_code)]
fn normalize_pos(pos: &mut glm::Vec2) {
    pos.x = (pos.x * 2_f32) / WINDOW_WIDTH as f32;
    pos.y = (pos.y * 2_f32) / WINDOW_HEIGHT as f32;
}

fn get_move_deltas(gui: &Gui, dt: &Duration, move_speed: &glm::Vec2) -> glm::Vec2 {

    let mut move_direction = glm::vec2(0.0, 0.0);

    if gui.is_key_pressed(glfw::Key::D) {
        move_direction.x = -1.0;
    }
    else if gui.is_key_pressed(glfw::Key::A) {
        move_direction.x = 1.0;
    }

    if gui.is_key_pressed(glfw::Key::W) {
        move_direction.y = -1.0;
    }
    else if gui.is_key_pressed(glfw::Key::S) {
        move_direction.y = 1.0;
    }

    move_speed.component_mul(&move_direction) * dt.as_secs_f32()

}
