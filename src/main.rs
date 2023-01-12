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

use egui_glfw_gl::egui;
use egui_glfw_gl::glfw;
use figure::Figure;
use image::{io::Reader, DynamicImage};
use nalgebra_glm as glm;

use screen::Screen;
use texture::Texture;

use gl::{self, types::{GLenum, GLuint, GLsizei, GLchar}};
use common::{WINDOW_WIDTH, WINDOW_HEIGHT};
use ui::Gui;

struct ParamsInput {
    max_iters_str: String,
    max_iters: u32,
    iterations: u32,
    color_jump: u32,
    fract_type: u32,
    pause_julia: bool,
    julia_speed: f32,
}

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

    let mut params_input = ParamsInput{
        max_iters_str: String::from("10000"),
        max_iters: 10000,
        iterations: 1000,
        color_jump: 100,
        fract_type: figure::FRACTAL_TYPE_MANDELBROT,
        pause_julia: false,
        julia_speed: 0.1,
    };
    let move_speed = glm::Vec2::new(
        WINDOW_WIDTH as f32 / 500.0, WINDOW_HEIGHT as f32 / 500.0
    );
    let mut figure = Figure::new();
    let mut scale = figure.get_scale();

    let mut show_animation = false;

    let mut start = Instant::now();
    let mut dt = Duration::from_secs_f32(1.0 / 60.0);

    let mut alpha: f32 = 0.0;

    while !gui.should_close_window() {

        gui.start_frame();

        screen.clear();

        let scroll_amount = gui.consume_scroll_amount();
        scale = scale + glm::Vec2::from_element(scroll_amount as f32 * scale.x * 0.1);
        figure.set_scale(scale);

        let dpos = get_move_deltas(&gui, &dt, &move_speed) / scale.x;
        figure.set_position(figure.get_position() + dpos);

        figure.set_iterations(params_input.iterations);
        figure.set_color_jump((params_input.color_jump as f32 * (scale.x + 1.0).ln()) as u32);

        figure.set_fractal_type(params_input.fract_type);

        if params_input.fract_type == figure::FRACTAL_TYPE_JULIA && !params_input.pause_julia {
            let julia_c = glm::vec2(alpha.cos(), alpha.sin()) * 0.7885;
            alpha += params_input.julia_speed * dt.as_secs_f32();
            alpha %= std::f32::consts::PI * 2.0;

            figure.set_julia_c(julia_c);
        }

        figure.render();

        render_gui(&gui, &mut params_input, &mut show_animation);

        gui.end_frame();

        dt = start.elapsed();
        start = Instant::now();
    }
}

fn render_gui(
    gui: &Gui, params_input: &mut ParamsInput, show_animation: &mut bool
) {

        gui.show(|ui| {

            ui.label("iterations:");
            ui.horizontal(|ui| {

                ui.set_max_width(ui.min_size().x);

                ui.add(egui::Slider::u32(
                    &mut params_input.iterations, 10..=params_input.max_iters
                ));

                ui.label("<");

                ui.text_edit_singleline(&mut params_input.max_iters_str);
                if let Result::Ok(i) = params_input.max_iters_str.parse::<u32>() {
                    params_input.max_iters = i;
                }
            });

            ui.label("Color jumps:");
            ui.add(egui::Slider::u32(
                &mut params_input.color_jump, 10..=100
            ));

            ui.separator();

            ui.label("Fractal type:");

            if ui.radio(
                params_input.fract_type == figure::FRACTAL_TYPE_MANDELBROT,
                "Mandelbrot",
            ).clicked() {
                params_input.fract_type = figure::FRACTAL_TYPE_MANDELBROT;
            }

            if ui.radio(
                params_input.fract_type == figure::FRACTAL_TYPE_JULIA,
                "Julia",
            ).clicked() {
                params_input.fract_type = figure::FRACTAL_TYPE_JULIA;
            }

            if params_input.fract_type == figure::FRACTAL_TYPE_JULIA {
                ui.separator();

                ui.label("Julia animation speed.");
                ui.add(egui::Slider::f32(
                    &mut params_input.julia_speed, 0.001..=1.0
                ));

                let mut pause_continue_button_str = "Pause";

                if params_input.pause_julia {
                    pause_continue_button_str = "Continue";
                }

                if ui.button(pause_continue_button_str).clicked() {
                    params_input.pause_julia = !params_input.pause_julia;
                }
            }
        });
}

fn read_texture(path: &str) -> Texture {

    let image = Reader::open(path)
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
                for pixel in row.rev() {
                    pixels.push((pixel[0], pixel[1], pixel[2], pixel[3]))
                }
            }
            Texture::new(width, height, texture::TexType::Color, Some(&pixels))
        },
        _ => panic!("Unexpected image format."),
    }
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

    move_speed.component_mul(&move_direction) * dt.as_secs_f32() * 0.5

}
