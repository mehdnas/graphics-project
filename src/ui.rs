
use std::{time::Instant, collections::VecDeque};
use std::sync::mpsc::Receiver;

use egui_glfw_gl as egui_backend;
use nalgebra_glm as glm;

use egui_backend::glfw as glfw;
use glfw::{Action, Context, Key, };

use egui_backend::egui as egui;
use egui::{vec2, Pos2, Rect};

pub struct Gui {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    painter: egui_backend::Painter,
    egui_ctx: egui::CtxRef,
    native_pixels_per_point: f32,
    egui_input_state: egui_backend::EguiInputState,
    start_time: Instant,
    scroll_amount: f64,
    cursor_left_presses: VecDeque<glm::Vec2>,
}

impl Gui {

    pub fn new(
        width: u32,
        height: u32,
        title: &str,
    ) -> Self {

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
        glfw.window_hint(glfw::WindowHint::Resizable(false));

        let (mut window, events) = glfw.create_window(
            width, height, title, glfw::WindowMode::Windowed
        ).unwrap();

        window.set_char_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);

        window.set_scroll_polling(true);

        window.make_current();

        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        gl::load_with(|proc_name| window.get_proc_address(proc_name));

        let painter = egui_backend::Painter::new(
            &mut window, width, height
        );

        let egui_ctx = egui::CtxRef::default();

        let (width, height) = window.get_framebuffer_size();
        let native_pixels_per_point = window.get_content_scale().0;

        let egui_input_state = egui_backend::EguiInputState::new(egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::new(0f32, 0f32),
                vec2(width as f32, height as f32) / native_pixels_per_point,
            )),
            pixels_per_point: Some(native_pixels_per_point),
            ..Default::default()
        });

        let start_time = Instant::now();

        Self {
            glfw,
            window,
            events,
            painter,
            egui_ctx,
            native_pixels_per_point,
            egui_input_state,
            start_time,
            scroll_amount: 0.0,
            cursor_left_presses: VecDeque::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get_window_size(&self) -> (i32, i32) {
        self.window.get_size()
    }

    pub fn should_close_window(&self) -> bool {
        self.window.should_close()
    }

    pub fn is_key_pressed(&self, key: glfw::Key) -> bool {
        self.window.get_key(key) == glfw::Action::Press
    }

    pub fn start_frame(&mut self) {

        self.egui_input_state.input.time = Some(
            self.start_time.elapsed().as_secs_f64()
        );
        self.egui_ctx.begin_frame(self.egui_input_state.input.take());

        //In egui 0.10.0 we seem to be losing the value to pixels_per_point,
        //so setting it every frame now.
        //TODO: Investigate if this is the right way.
        self.egui_input_state.input.pixels_per_point = Some(self.native_pixels_per_point);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

    }

    pub fn end_frame(&mut self) {

        let (egui_output,  paint_cmds) = self.egui_ctx.end_frame();

        if !egui_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(&mut self.egui_input_state, egui_output.copied_text);
        }

        let paint_jobs = self.egui_ctx.tessellate(paint_cmds);


        self.painter.paint_jobs(
            None, paint_jobs, &self.egui_ctx.texture(), self.native_pixels_per_point
        );

        self.handle_window_events();

        self.window.swap_buffers();
        self.glfw.poll_events();
    }

    pub fn show(&self, gui_fn: impl FnOnce(&mut egui::Ui)) {
        egui_backend::egui::Window::new("GUI").show(&self.egui_ctx, gui_fn);
    }

    pub fn consume_scroll_amount(&mut self) -> f64 {
        let used_scroll_amount = self.scroll_amount;
        self.scroll_amount = 0.0;
        used_scroll_amount
    }

    pub fn consume_cursor_left_press_pos(&mut self) -> Option<glm::Vec2> {
        self.cursor_left_presses.pop_front()
    }

    pub fn get_cursor_pos(&self) -> glm::Vec2 {
        let (x, y) = self.window.get_cursor_pos();
        glm::vec2(x as f32, y as f32)
    }

    fn handle_window_events(&mut self) {

        for (_, event) in glfw::flush_messages(&self.events) {

            match event {

                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true);
                },

                glfw::WindowEvent::Scroll(_, y) => {
                    if !self.egui_ctx.is_pointer_over_area() {
                        self.scroll_amount += y
                    }
                }

                glfw::WindowEvent::MouseButton(
                    glfw::MouseButtonLeft, glfw::Action::Press, _
                ) => {
                    if !self.egui_ctx.is_pointer_over_area() {
                        let (x, y) = self.window.get_cursor_pos();
                        self.cursor_left_presses.push_back(
                            glm::vec2(x as f32, y as f32)
                        );
                    }
                }

                _ => {}
            }
            egui_backend::handle_event(event, &mut self.egui_input_state);
        }
    }
}

impl Drop for Gui {
    fn drop(&mut self) {
        self.painter.cleanup();
    }
}
