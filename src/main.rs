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

use figure::Figure;
use image::{io::Reader, DynamicImage};

use screen::Screen;
use texture::Texture;

use gl::{self, types::{GLenum, GLuint, GLsizei, GLchar}};
use common::{WINDOW_WIDTH, WINDOW_HEIGHT};
use ui::Gui;
use nalgebra_glm as glm;

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

    let texture = read_texture("car.png");
    let mut figure = Figure::new(texture);

    while !gui.should_close_window() {

        gui.start_frame();

        screen.clear();

        figure.render();

        figure.set_scale(0.5);
        figure.set_position(&glm::vec2(0.5, 0.5));

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

fn read_texture(path: &str) -> Texture {

    let mut image = Reader::open(path)
        .expect("ERROR: Bad image path.")
        .with_guessed_format()
        .expect("ERROR: Probably bad format.")
        .decode()
        .expect("ERROR: Could not decode image.");

    match image {
        DynamicImage::ImageRgba8(buffer) => {
            let width = buffer.width() as u16;
            let height = buffer.height() as u16;
            let mut pixels: Vec<(u8, u8, u8, u8)> = Vec::new();
            for row in buffer.rows().rev() {
                for pixel in row {
                    pixels.push((pixel[0], pixel[1], pixel[2], pixel[3]))
                }
            }
            Texture::new(width, height, texture::TexType::Color, Some(&pixels))
        },
        _ => panic!("Unexpected image format."),
    }
}
