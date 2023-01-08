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
use egui::TextBuffer;
use egui::widgets;
use figure::Figure;
use image::{io::Reader, DynamicImage};

use screen::Screen;
use texture::Texture;

use gl::{self, types::{GLenum, GLuint, GLsizei, GLchar}};
use common::{WINDOW_WIDTH, WINDOW_HEIGHT};
use ui::Gui;
use nalgebra_glm as glm;

struct TransformationsInput {
    pub scale_x_str: String,
    pub scale_y_str: String,
    pub shearing_x_str: String,
    pub shearing_y_str: String,
    pub rotations_str: String,
    pub position_x_str: String,
    pub position_y_str: String,
    pub scale: glm::Vec2,
    pub shearing: glm::Vec2,
    pub rotation: f32,
    pub position: glm::Vec2,
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

    let screen = Screen::default();

    let mut transformations = TransformationsInput{
        scale_x_str: String::from("1.0"),
        scale_y_str: String::from("1.0"),
        shearing_x_str: String::from("0.0"),
        shearing_y_str: String::from("0.0"),
        rotations_str: String::from("0.0"),
        position_x_str: String::from("0.0"),
        position_y_str: String::from("0.0"),
        scale: glm::vec2(1.0, 1.0),
        shearing: glm::vec2(0.0, 0.0),
        rotation: 0.0,
        position: glm::vec2(0.0, 0.0),
    };
    let texture = read_texture("car.png");
    let mut figure = Figure::new(texture);

    let mut show_animation = false;

    while !gui.should_close_window() {

        gui.start_frame();

        screen.clear();

        if show_animation {
            animation(&mut gui, &mut figure);
            show_animation = false;
        }

        figure.set_scale(transformations.scale);
        figure.set_position(&transformations.position);
        figure.set_shearing(&transformations.shearing);
        figure.set_rotation(transformations.rotation);

        figure.render();

        render_gui(&gui, &mut transformations, &mut show_animation);

        gui.end_frame();
    }
}

fn animation(gui: &mut Gui, figure: &mut Figure) {

    let mut start = Instant::now();
    let mut dt = Duration::from_secs_f32(1.0 / 60.0);

    figure.set_position(&glm::vec2(
        -((WINDOW_HEIGHT - 100) as f32 / WINDOW_HEIGHT as f32),
        -((WINDOW_WIDTH - 100) as f32 / WINDOW_WIDTH as f32)
    ));

    figure.set_scale(glm::vec2(
        100.0 / WINDOW_HEIGHT as f32,
        100.0 / WINDOW_WIDTH as f32
    ));

    let mut end_animation = false;

    let mut acceleration = 0.0;
    let mut speed = 0.0;

    while !end_animation && !gui.should_close_window() {

        gui.start_frame();

        figure.render();

        let position = 

        animation_gui(gui, &mut end_animation);

        figure.set_position(&(figure.get_position() + glm::vec2(
            (speed / WINDOW_WIDTH as f32) * dt.as_secs_f32(), 0.0
        )));

        gui.end_frame();

        dt = start.elapsed();
        start = Instant::now();
    }
}

fn animation_gui(gui: &Gui, end_animation: &mut bool) {

    gui.show(|ui| {
        if ui.button("End animation").clicked() {
            *end_animation = true;
        }
    });
}

fn render_gui(
    gui: &Gui, transformations: &mut TransformationsInput, show_animation: &mut bool
) {

        gui.show(|ui| {

            ui.label("Translation:");

            ui.horizontal(|ui| {
                ui.set_max_size(egui::vec2(100.0, 10.0));
                ui.label("x:");
                ui.text_edit_singleline(&mut transformations.position_x_str);
                if let Result::Ok(x) = transformations.position_x_str.parse::<f32>() {
                    transformations.position.x = x / WINDOW_WIDTH as f32;
                }

                ui.separator();
                ui.set_max_size(egui::vec2(100.0, 10.0));
                ui.label("y:");
                ui.text_edit_singleline(&mut transformations.position_y_str);
                if let Result::Ok(y) = transformations.position_y_str.parse::<f32>() {
                    transformations.position.y = y / WINDOW_HEIGHT as f32;
                }
            });

            ui.separator();

            ui.label("Scaling:");

            ui.horizontal(|ui| {
                ui.set_max_size(egui::vec2(100.0, 10.0));
                ui.label("x:");
                ui.text_edit_singleline(&mut transformations.scale_x_str);
                if let Result::Ok(x) = transformations.scale_x_str.parse::<f32>() {
                    transformations.scale.x = x;
                }

                ui.separator();
                ui.set_max_size(egui::vec2(100.0, 10.0));
                ui.label("y:");
                ui.text_edit_singleline(&mut transformations.scale_y_str);
                if let Result::Ok(y) = transformations.scale_y_str.parse::<f32>() {
                    transformations.scale.y = y;
                }
            });

            ui.separator();

            ui.label("Shearing:");

            ui.horizontal(|ui| {
                ui.set_max_size(egui::vec2(100.0, 10.0));
                ui.label("x:");
                ui.text_edit_singleline(&mut transformations.shearing_x_str);
                if let Result::Ok(x) = transformations.shearing_x_str.parse::<f32>() {
                    transformations.shearing.x = x;
                }

                ui.separator();
                ui.set_max_size(egui::vec2(100.0, 10.0));
                ui.label("y:");
                ui.text_edit_singleline(&mut transformations.shearing_y_str);
                if let Result::Ok(y) = transformations.shearing_y_str.parse::<f32>() {
                    transformations.shearing.y = y;
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.set_max_size(egui::vec2(100.0, 10.0));
                ui.label("Rotation:");
                ui.text_edit_singleline(&mut transformations.rotations_str);
                if let Result::Ok(r) = transformations.rotations_str.parse::<f32>() {
                    transformations.rotation = (r * std::f64::consts::PI as f32) / 180.0;
                }
            });

            ui.separator();

            if ui.button("show_animation").clicked() {
                *show_animation = true;
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
