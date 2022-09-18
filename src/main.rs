
#![feature(core_c_str)]

use core::ffi::CStr;
use std::ffi::c_void;

use quad::Quad;
use shader_program::ShaderProgram;

use gl::{self, types::{GLenum, GLuint, GLsizei, GLchar}};
use vao::{Vertex, Vao};

mod ui;
mod quad;
mod vao;
mod shader_program;

const WINDOW_HEIGHT: i32 = 600;
const WINDOW_WIDTH: i32 = 800;

extern "system" fn gl_debug_proc(
    source: GLenum,
    gltype: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    user_param: *mut c_void
) {
    if severity != gl::DEBUG_SEVERITY_NOTIFICATION {
        let message_c_str: &CStr = unsafe {CStr::from_ptr(message)};
        let message_slice: &str = message_c_str.to_str().unwrap();
        eprintln!("GL CALLBACK: {}", message_slice);
    }
}

fn main() {

    let mut gui = ui::Gui::new(
        WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, "Graficos por Computador"
    );

    unsafe {
        gl::DebugMessageCallback(Some(gl_debug_proc), 0 as *const c_void);
        gl::Enable(gl::DEBUG_OUTPUT);
    }

    let quad = Quad::default();
    let shader_program = ShaderProgram::new(
        "src/shaders/vertex.glsl",
        "src/shaders/fragment.glsl",
    );

    unsafe {
        gl::Viewport(0, 0, WINDOW_HEIGHT, WINDOW_WIDTH);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    while !gui.should_close_window() {

        gui.start_frame();

        quad.render(&shader_program);

        gui.show(|ui| {
            ui.separator();
            ui.label(" ");
            ui.label(" ");
            ui.label(" ");
        });

        gui.end_frame();
    }
}

