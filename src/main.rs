
use std::time::Instant;

extern crate gl;
use egui_backend::EguiInputState;
use gl::types::*;


use egui_glfw_gl as egui_backend;
use egui_backend::egui as egui;
use egui_backend::glfw as glfw;
use egui::{vec2, Color32, Image, Pos2, Rect};
use glfw::{Action, Context, Key};

mod triangle;

const WINDOW_HEIGHT: i32 = 600;
const WINDOW_WIDTH: i32 = 800;
const PIC_WIDTH: i32 = 320;
const PIC_HEIGHT: i32 = 192;

fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, events) = glfw.create_window(
        WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, "Graficos por Computador",
        glfw::WindowMode::Windowed
    ).expect("Failed to create GLFW window");

    window.set_char_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);

    window.make_current();

    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    gl::load_with(|proc_name| window.get_proc_address(proc_name));

    let mut painter = egui_backend::Painter::new(
        &mut window, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32
    );
    let mut egui_ctx = egui::CtxRef::default();
    let (width, height) = window.get_framebuffer_size();
    let native_pixels_per_point = window.get_content_scale().0;

    let mut egui_input_state = egui_backend::EguiInputState::new(egui::RawInput {
        screen_rect: Some(Rect::from_min_size(
            Pos2::new(0f32, 0f32),
            vec2(width as f32, height as f32) / native_pixels_per_point,
        )),
        pixels_per_point: Some(native_pixels_per_point),
        ..Default::default()
    });

    let start_time = Instant::now();
    let mut srgba: Vec<Color32> = Vec::new();

    for _ in 0..PIC_HEIGHT {
        for _ in 0..PIC_WIDTH  {
            srgba.push(Color32::BLACK);
        }
    }

    let plot_tex_id =
        painter.new_user_texture((PIC_WIDTH as usize, PIC_HEIGHT as usize), &srgba, false);

    let mut sine_shift = 0f32;

    let mut amplitud: f32 = 50f32;
    let mut test_str: String =
        "A text box to write in. Cut, copy, paste commands are available.".to_owned();

    let triangle = triangle::Triangle::new();
    let mut quit = false;

    unsafe {
        gl::Viewport(0, 0, WINDOW_HEIGHT, WINDOW_WIDTH);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    while !window.should_close() {

        egui_input_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_input_state.input.take());


        //In egui 0.10.0 we seem to be losing the value to pixels_per_point,
        //so setting it every frame now.
        //TODO: Investigate if this is the right way.
        egui_input_state.input.pixels_per_point = Some(native_pixels_per_point);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        triangle.draw();

        let mut srgba: Vec<Color32> = Vec::new();
        let mut angle = 0f32;

        for y in 0..PIC_HEIGHT {
            for x in 0..PIC_WIDTH {
                srgba.push(Color32::BLACK);
                if y == PIC_HEIGHT - 1 {
                    let y = amplitud * (angle * 3.142f32 / 180f32 + sine_shift).sin();
                    let y = PIC_HEIGHT as f32 / 2f32 - y;
                    srgba[(y as i32 * PIC_WIDTH + x) as usize] = Color32::YELLOW;
                    angle += 360f32 / PIC_WIDTH as f32;
                }
            }
        }
        sine_shift += 0.1f32;

        painter.update_user_texture_data(plot_tex_id, &srgba);

        egui_backend::egui::Window::new("EGUI with GLFW").show(&egui_ctx, |ui| {
            ui.add(Image::new(plot_tex_id, vec2(PIC_WIDTH as f32, PIC_HEIGHT as f32)));
            ui.separator();
            ui.label("A simple sine wave plotted into a GL texture then blitted to an egui managed Image.");
            ui.label(" ");
            ui.text_edit_multiline(& mut test_str);
            ui.label(" ");

            ui.add(egui::Slider::new(&mut amplitud, 0.0..=50.0).text("Amplitud"));
            ui.label(" ");
            if ui.button("Quit").clicked() {
                quit = true;
            }
        });

        let (egui_output,  paint_cmds) = egui_ctx.end_frame();

        if !egui_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(&mut egui_input_state, egui_output.copied_text);
        }

        let paint_jobs = egui_ctx.tessellate(paint_cmds);


        painter.paint_jobs(None, paint_jobs, &egui_ctx.texture(), native_pixels_per_point);

        for (_, event) in glfw::flush_messages(&events) {
            print!("{:?}", event);

            handle_window_event(&mut window, event, &mut egui_input_state);
        }

        window.swap_buffers();
        glfw.poll_events();

        if quit {window.set_should_close(true)};
    }
}


fn handle_window_event(
    window: &mut glfw::Window, event: glfw::WindowEvent, egui_input_state: &mut EguiInputState
) {
    match event {

        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        },

        _ => {egui_backend::handle_event(event, egui_input_state);},
    }
}
