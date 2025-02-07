use settings::*;
use math::*;

use window::{Window, Events, Camera};
use graphics::{load_shader, VoxelRenderer};
use loaders::{load_texture};
use voxels::{Chunk, Chunks, chunk::CHUNK_D, chunk::CHUNK_W, chunk::CHUNK_H};
use crate::graphics::mesh::Mesh;

mod window;
mod graphics;
mod loaders;
mod voxels;
mod settings;
mod math;


// const VERTICES: [f32; 30] = [
//     -1.0f32, -1.0f32, 0.0f32, 0.0f32, 0.0f32,
//     1.0f32, -1.0f32, 0.0f32, 1.0f32, 0.0f32,
//     -1.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32,
//
//     1.0f32, -1.0f32, 0.0f32, 1.0f32, 0.0f32,
//     1.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32,
//     -1.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32,
// ];


const vertices: [f32; 8] = [
    -0.01f32, -0.01f32,
    0.01f32, 0.01f32,

    -0.01f32, 0.01f32,
    0.01f32, -0.01f32
];

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const TITLE: &str = "Voxel_Craft";

#[allow(non_upper_case_globals)]
const attrs: [i32; 2] = [2, 0];


fn main() {
    let mut window = Window::init(WIDTH, HEIGHT, TITLE).unwrap();
    let mut events = Events::init();

    events.setting(&mut window);

    let shader = load_shader("res/main.glslv","res/main.glslf").expect("Failed to load shader");
    let crosshair_shader = load_shader(
        "res/crosshair.glslv",
        "res/crosshair.glslf"
    ).expect("Failed to load shader");

    let texture = load_texture("res/block.png").expect("Failed to load texture");

    let mut chunks = Chunks::new(5, 1, 5);
    let mut meshes = Vec::with_capacity(chunks.volume);
    let mut renderer = VoxelRenderer::new(1024*1024*8);


    for i in 0..chunks.volume {
        let mesh = renderer.render(&*chunks.chunks[i], &vec![]);
        meshes.push(mesh);
    }


    window.clear_color(1.0, 1.0, 1.0, 1.0);

    window.setting_gl();

    let mut crosshair = Mesh::new(vertices.as_ptr(), 4, attrs.as_ptr());

    let mut camera = Camera::init(Vec3::new(0.0, 0.0, 1.0), 70.0_f32.to_radians());

    let model = Mat4::IDENTITY;

    let mut last_time = window.glfw.get_time();
    let mut _delta:f64 = 0.0;

    let speed:f32 = 5.0f32;

    let mut cam_x = 0.0;
    let mut cam_y = 0.0;

    while !window.should_close() {
        let current_time = window.glfw.get_time();
        _delta = current_time - last_time;
        last_time = current_time;

        if events.jpressed(ESCAPE) {
            window.close();
        }

        // if events.jclicked(LCM){
        //     window.clear_color(0.0, 0.0, 0.0, 0.0);
        // }
        //
        // if events.jclicked(PCM){
        //     window.clear_color(0.4, 0.8, 0.6, 0.5);
        // }
        //
        // if events.jclicked(SCM){
        //     window.clear_color(1.0, 1.0, 1.0, 0.5);
        // }

        if events.pressed(Q){
            camera.position.z += _delta as f32 * speed;
        }

        if events.pressed(E){
            camera.position.z -= _delta as f32 * speed;
        }

        if events.pressed(A){
            camera.position -= camera.right * _delta as f32 * speed;
        }

        if events.pressed(D){
            camera.position += camera.right * _delta as f32 * speed;
        }

        if events.pressed(S){
            camera.position -= camera.up * _delta as f32 * speed;
        }

        if events.pressed(W){
            camera.position += camera.up * _delta as f32 * speed;
        }

        if events.jpressed(TAB){
            window.window.set_cursor_mode(events.toggle_cursor());
        }

        if events.cursor_locked {
            cam_y += -events.delta_y / (window.height() as f32) * 2.0;
            cam_x += -events.delta_x / (window.height() as f32) * 2.0;

             //    cam_y < -90.0_f32.to_radians() {   // ????
             //      cam_y = -90.0_f32.to_radians();
             // }
             if cam_y > 89.0_f32.to_radians() {
                 cam_y = 89.0_f32.to_radians();
             }

            camera.rotation = Quat::IDENTITY;
            camera.rotate(cam_y, cam_x, 0.0);
        }


        let mut closes: Vec<Option<Chunk>> = vec![None; 27];

        for i in 0..chunks.volume {
            if let Some(chunk) = chunks.chunks.get_mut(i) {
                if !chunk.modified {
                    continue;
                }
                chunk.modified = false;
            }
            let chunk = &chunks.chunks[i];

            // if let Some(mesh) = meshes[i].take() {
            //     // Освобождаем ресурсы меша
            //     drop(mesh);
            // }

            // Инициализируем массив closes снова
            for elem in &mut closes {
                *elem = None;
            }

            for j in 0..chunks.volume {
                let other = &chunks.chunks[j];
                let ox = other.x - chunk.x;
                let oy = other.y - chunk.y;
                let oz = other.z - chunk.z;

                if ox.abs() > 1 || oy.abs() > 1 || oz.abs() > 1 {
                    continue;
                }

                let index = ((oy + 1) * 3 + (oz + 1)) * 3 + (ox + 1);
                closes[index as usize] = Some(*other.clone());
            }

            let mesh = renderer.render(chunk.as_ref(), &closes);
            meshes[i] = mesh;
        }

        window.gl_clear();

        shader.use_shader();
        shader.uniform_matrix("preview", camera.get_projection(window.width() as f32, window.height() as f32) * camera.get_view());
        texture.bind();

        let mut model = Mat4::IDENTITY;
        model *= Mat4::from_translation(vec3(0.5, 0.0, 0.0));

        for i in 0..chunks.volume {
            let chunk = &chunks.chunks[i];
            let mesh = &meshes[i];
            model =
                Mat4::IDENTITY *
                    Mat4::from_translation(
                        vec3(
                            ((chunk.x * CHUNK_W) as f32) + 0.5,
                            ((chunk.y * CHUNK_H) as f32) + 0.5,
                            ((chunk.z * CHUNK_D) as f32) + 0.5
                        )
                    );
            shader.uniform_matrix("model", model);
            mesh.draw(TRIANGLES);

        }


        crosshair_shader.use_shader();
        crosshair.draw(LINES);


        window.swap_buffers();
        window.poll_events();
        events.pull_events(&mut window);
    }

    window.terminate();
}