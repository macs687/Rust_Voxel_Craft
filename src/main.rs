use std::mem;
use gl::TRIANGLES;
use gl::types::{GLfloat, GLint, GLsizeiptr, GLuint};
use glfw::ffi::{KEY_A, KEY_D, KEY_E, KEY_ESCAPE, KEY_Q, KEY_S, KEY_TAB, KEY_W, MOUSE_BUTTON_LEFT, MOUSE_BUTTON_MIDDLE, MOUSE_BUTTON_RIGHT};
use window::{events::Events, Window, Camera};
use graphics::load_shader;
use loaders::{png_loading};
use crate::loaders::png_loading::load_texture;
use glam::*;
use glfw::*;

mod window;
mod graphics;
mod loaders;


const VERTICES: [f32; 30] = [
    -1.0f32, -1.0f32, 0.0f32, 0.0f32, 0.0f32,
    1.0f32, -1.0f32, 0.0f32, 1.0f32, 0.0f32,
    -1.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32,

    1.0f32, -1.0f32, 0.0f32, 1.0f32, 0.0f32,
    1.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32,
    -1.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32,
];

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const TITLE: &str = "Voxel_Craft";

#[allow(non_upper_case_globals)]
const attrss: [i32; 2] = [2, 0];


// pub fn new(buffer: *const f32, vertices: usize, attrs: *const i32) {
//     let mut _vertex_size = 0;
//     let mut i = 0;
//     while unsafe { *attrs.offset(i) } != 0 {
//         _vertex_size += unsafe { *attrs.offset(i) as usize };
//         i += 1;
//     }
//
//     let mut vao = 0;
//     let mut vbo = 0;
//     unsafe {
//         gl::GenVertexArrays(1, &mut vao);
//         gl::GenBuffers(1, &mut vbo);
//
//         gl::BindVertexArray(vao);
//         gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
//         gl::BufferData(gl::ARRAY_BUFFER, (std::mem::size_of::<f32>() * _vertex_size * vertices) as GLsizeiptr, buffer as *const std::ffi::c_void, gl::STATIC_DRAW, );
//
//         let mut offset = 0;
//         let mut i = 0;
//         while *attrs.offset(i) != 0 {
//             let size = *attrs.offset(i) as GLint;
//             gl::VertexAttribPointer(
//                 i as GLuint,
//                 size,
//                 gl::FLOAT,
//                 gl::FALSE,
//                 (_vertex_size * std::mem::size_of::<f32>()) as GLint,
//                 (offset * std::mem::size_of::<f32>()) as *const std::ffi::c_void,
//             );
//             gl::EnableVertexAttribArray(i as GLuint);
//             offset += size as usize;
//             i += 1;
//         }
//
//         gl::BindVertexArray(0);
//     }
// }



fn main() {
    let mut window = Window::init(WIDTH, HEIGHT, TITLE).unwrap();
    let mut events = Events::init();

    events.setting(&mut window);

    let shader = load_shader("res/main.glslv","res/main.glslf").expect("Failed to load shader");

    let texture = load_texture("res/img.png").expect("Failed to load texture");


    //new(VERTICES.as_ptr(), 6, attrss.as_ptr());

    let mut _vertex_size = 0;
    let mut i = 0;
    while unsafe { *attrss.as_ptr().offset(i) } != 0 {
        _vertex_size += unsafe { *attrss.as_ptr().offset(i) as usize };
        i += 1;
    }

    let mut vao = 0;
    let mut vbo = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (std::mem::size_of::<f32>() * _vertex_size * 6) as GLsizeiptr, VERTICES.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW, );




        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as GLint,
            (0 * std::mem::size_of::<f32>()) as *const std::ffi::c_void,
        );

        gl::EnableVertexAttribArray(0);


        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as GLint,
            (3 * std::mem::size_of::<f32>()) as *const std::ffi::c_void,
        );

        gl::EnableVertexAttribArray(1);


        gl::BindVertexArray(0);
    }



    window.clear_color(1.0, 1.0, 1.0, 1.0);
    // gl::Enable(DEPTH_TEST);
    // gl::Enable(gl::CULL_FACE);
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }


    let mut camera = Camera::init(Vec3::new(0.0, 0.0, 1.0), 70.0_f32.to_radians());








    let mut model = Mat4::IDENTITY;
    //model = Mat4::from_scale(glam::vec3(2.0f32, 2.0f32, 2.0f32));
    //model = Mat4::from_translation(vec3(1.0f32, 1.0f32, 1.0f32));


    let mut last_time = window.glfw.get_time();
    let mut _delta:f64 = 0.0;

    let speed:f32 = 5.0f32;

    let mut cam_x = 0.0;
    let mut cam_y = 0.0;

    while !window.should_close() {
        let current_time = window.glfw.get_time();
        _delta = current_time - last_time;
        last_time = current_time;

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

        if events.pressed(KEY_Q){
            println!("Нажата Q ");
            camera.position.z += _delta as f32 * speed;
        }

        if events.pressed(KEY_E){
            println!("Нажата E ");
            camera.position.z -= _delta as f32 * speed;
        }

        if events.pressed(KEY_A){
            println!("Нажата A ");
            camera.position -= camera.right * _delta as f32 * speed;
        }

        if events.pressed(KEY_D){
            println!("Нажата D ");
            camera.position += camera.right * _delta as f32 * speed;
        }

        if events.pressed(KEY_S){
            println!("Нажата S ");
            camera.position -= camera.up * _delta as f32 * speed;
        }

        if events.pressed(KEY_W){
            println!("Нажата W ");
            camera.position += camera.up * _delta as f32 * speed;
        }

        if events.jpressed(KEY_TAB){
            println!("Нажата TAB");
            window.window.set_cursor_mode(events.toggle_cursor());
        }

        if events.cursor_locked {
            cam_y += -events.delta_y / (window.height() as f32) * 2.0;
            cam_x += -events.delta_x / (window.height() as f32) * 2.0;

             if cam_y < -89.0_f32.to_radians() {   // ????
                 cam_y = -89.0_f32.to_radians();
             }
             if cam_y > 89.0_f32.to_radians() {
                 cam_y = 89.0_f32.to_radians();
             }

            camera.rotation = Quat::IDENTITY;
            camera.rotate(cam_y, cam_x, 0.0);
        }


        //camera.rotate(events.delta_x / (window.height() as f32), -events.delta_x / (window.height() as f32), 0.0);
        


        window.gl_clear();

        shader.use_shader();
        shader.uniform_matrix("model", model);
        shader.uniform_matrix("projview", camera.get_projection(window.width() as f32, window.height() as f32) * camera.get_view());
        texture.bind();




        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }


        window.swap_buffers();
        window.poll_events();
        events.pull_events(&mut window);
    }

    window.terminate();
}