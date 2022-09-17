
extern crate gl;

mod triangle;
mod ui;
mod quad;

const WINDOW_HEIGHT: i32 = 600;
const WINDOW_WIDTH: i32 = 800;

fn main() {

    let mut gui = ui::Gui::new(
        WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, "Graficos por Computador"
    );

    let triangle = triangle::Triangle::new();

    unsafe {
        gl::Viewport(0, 0, WINDOW_HEIGHT, WINDOW_WIDTH);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    while !gui.should_close_window() {

        gui.start_frame();

        triangle.draw();

        gui.show(|ui| {
            ui.separator();
            ui.label(" ");
            ui.label(" ");
            ui.label(" ");
        });

        gui.end_frame();
    }
}

