extern crate gl;
// use gl::types::*;

use glfw::{Action, Context, Key};

use egui_glfw_gl as egui_backend;
use egui;

fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    const WINDOW_HEIGHT: i32 = 640;
    const WINDOW_WIDTH: i32 = 480;

    let (mut window, events) = glfw.create_window(
        WINDOW_HEIGHT as u32, WINDOW_WIDTH as u32, "Graficos por Computador",
        glfw::WindowMode::Windowed
    ).expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|proc_name| window.get_proc_address(proc_name));

    let mut painter = egui_backend::Painter::new(
        &mut window, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32
    );
    let native_pixels_per_point = window.get_content_scale().0;

    unsafe {gl::Viewport(0, 0, WINDOW_HEIGHT, WINDOW_WIDTH)}
    unsafe {gl::ClearColor(0.0, 0.0, 0.0, 0.0)}

    while !window.should_close() {

        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            print!("{:?}", event);

            handle_window_event(&mut window, event);
        }



        window.swap_buffers();
        unsafe {gl::Clear(gl::COLOR_BUFFER_BIT)}
    }
}


fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {

        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        },

        _ => {},
    }
}
