use glfw::ffi::{KEY_ESCAPE, MOUSE_BUTTON_LEFT, MOUSE_BUTTON_MIDDLE, MOUSE_BUTTON_RIGHT};
use window::{events::Events, Window};

mod window;


const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const TITLE: &str = "Voxel_Craft";

fn main() {
    let mut window = Window::init(WIDTH, HEIGHT, TITLE).unwrap();
    let mut events = Events::init();

    events.setting(&mut window);
    




    window.clear_color(1.0, 1.0, 1.0, 1.0);
    // gl::Enable(DEPTH_TEST);
    // gl::Enable(gl::CULL_FACE);
    // gl::Enable(gl::BLEND);
    // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);




    while !window.should_close() {
        if events.jpressed(KEY_ESCAPE) {
            window.close();
        }

        if events.jclicked(MOUSE_BUTTON_LEFT){
            window.clear_color(0.0, 0.0, 0.0, 0.0);
        }

        if events.jclicked(MOUSE_BUTTON_RIGHT){
            window.clear_color(0.4, 0.8, 0.6, 0.5);
        }

        if events.jclicked(MOUSE_BUTTON_MIDDLE){
            window.clear_color(1.0, 1.0, 1.0, 0.5);
        }
        
            /// события
        
        
        
        


        window.gl_clear();

        // шейдеры



        window.swap_buffers();
        window.poll_events();
        events.pull_events(&mut window);
    }
    window.terminate();
}