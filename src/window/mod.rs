pub mod events;
mod camera;
pub use camera::Camera;

extern crate gl;

use gl::DEPTH_BUFFER_BIT;
use gl::types::*;

extern crate glfw;
use glfw::{Glfw, PWindow, GlfwReceiver, WindowEvent, fail_on_errors, Action, Context, Key};
use glfw::ffi::{glfwTerminate};

pub struct Window{
    pub glfw: Glfw,
    pub window: PWindow,
    pub receiver: GlfwReceiver<(f64, WindowEvent)>,

}


impl Window {
    pub fn init(width: u32, height: u32, title: &str) -> Result<Self, String> {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::Resizable(true));

        let (mut window, events) = match glfw.create_window(width, height, title, glfw::WindowMode::Windowed) {
            Some((window, events)) => (window, events),
            None => {
                eprintln!("Failed to create GLFW window.");
                std::process::exit(1);
            }
        };

        window.make_current();
        gl::load_with(|s| window.get_proc_address(s) as *const _);
        window.set_key_polling(true);

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        Ok(Self {glfw, window, receiver: events})
    }

    pub fn terminate(&mut self) {
        unsafe { glfwTerminate() }
    }

    pub fn swap_buffers(&mut self){
        self.window.swap_buffers();
    }

    pub fn should_close(&mut self) -> bool {
        self.window.should_close()
    }

    pub fn close(&mut self){
        self.window.set_should_close(true);
    }

    pub fn poll_events(&mut self){
        self.glfw.poll_events();
    }

    pub fn gl_clear(&mut self){
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        }
    }

    pub fn clear_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32){
        unsafe { gl::ClearColor(red, green, blue, alpha) }
    }

    pub fn width(&self) -> i32{
        self.window.get_size().0
    }

    pub fn height(&self) -> i32{
        self.window.get_size().1
    }
}