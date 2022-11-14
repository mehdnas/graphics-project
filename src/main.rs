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
mod figure;

use nalgebra_glm as glm;

use screen::Screen;

use gl::{self, types::{GLenum, GLuint, GLsizei, GLchar}};
use common::{WINDOW_WIDTH, WINDOW_HEIGHT};
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

    let mut screen = Screen::default();

    let mut start = Instant::now();
    let mut dt = Duration::from_secs_f32(1.0 / 60.0);

    while !gui.should_close_window() {

        gui.start_frame();

        screen.clear();

        screen.render_used_texture();

        render_gui(&gui);

        gui.end_frame();

        dt = start.elapsed();
        start = Instant::now();
    }
}

fn render_gui(
    gui: &Gui,
) {

        gui.show(|ui| {

            ui.separator();

        });

}

