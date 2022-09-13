extern crate gl;
use gl::types::*;

extern crate glfw;
use glfw::{Action, Context, Key};


fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(
        300, 300, "Graficos por Computador", glfw::WindowMode::Windowed
    ).expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    while !window.should_close() {
        window.swap_buffers();

        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            print!("{:?}", event);

            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _)
                    => {
                        window.set_should_close(true)
                    },
                _ => {},
            }
        }
    }
}
