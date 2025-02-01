use window::{Window};

mod window;


const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const TITLE: &str = "Voxel_Craft";

fn main() {
    let mut window = Window::init(WIDTH, HEIGHT, TITLE).unwrap();

    unsafe {
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        // gl::Enable(DEPTH_TEST);
        // gl::Enable(gl::CULL_FACE);
        // gl::Enable(gl::BLEND);
        // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }



    while !window.should_close() {
        window.poll_events();




        window.swap_buffers();
    }

    window.terminate();
}