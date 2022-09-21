use core::ffi::CStr;
use std::ffi::c_void;

use nalgebra_glm as glm;

mod ui;
mod quad;
mod vao;
mod shader_program;
mod framebuffer;
mod texture;

use quad::Quad;
use shader_program::ShaderProgram;

use gl::{self, types::{GLenum, GLuint, GLsizei, GLchar}};
use framebuffer::Framebuffer;

const WINDOW_HEIGHT: i32 = 600;
const WINDOW_WIDTH: i32 = 800;

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
        WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, "Graficos por Computador"
    );

    unsafe {
        gl::DebugMessageCallback(Some(gl_debug_proc), 0 as *const c_void);
        gl::Enable(gl::DEBUG_OUTPUT);
    }

    let quad = Quad::default();
    let noise_shader_program = ShaderProgram::new(
        "src/shaders/noise_vertex.glsl",
        "src/shaders/noise_fragment.glsl"
    );
    let screen_shader_program = ShaderProgram::new(
        "src/shaders/screen_vertex.glsl",
        "src/shaders/screen_fragment.glsl"
    );
    let framebuffer = Framebuffer::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);

    unsafe {
        gl::Viewport(0, 0, WINDOW_HEIGHT, WINDOW_WIDTH);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    let mut transform = glm::identity::<f32, glm::U3>();
    let mut scale = 1.0;

    while !gui.should_close_window() {

        gui.start_frame();

        framebuffer.bind();

        quad.render(&noise_shader_program);

        framebuffer.unbind();

        unsafe {
            gl::BindTexture(
                gl::TEXTURE_2D,
                framebuffer.get_color_attachment_id()
            );
        }

        screen_shader_program.bind();

        screen_shader_program.set_uniform_mat3("transform", &transform);

        quad.render(&screen_shader_program);

        gui.show(|ui| {
            ui.separator();
            ui.label(" ");
            if ui.button("zoom in").clicked() {
                scale += 0.1;
                transform = glm::diagonal3x3(&glm::vec3(scale, scale, 1.0));
            }
            ui.label(" ");
            if ui.button("zoom out").clicked() {
                scale -= 0.1;
                transform = glm::diagonal3x3(&glm::vec3(scale, scale, 1.0));
            }
            ui.label(" ");
        });

        gui.end_frame();
    }
}

