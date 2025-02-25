use settings::*;
use math::*;

use window::{Window, Events, Camera};
use graphics::{load_shader, VoxelRenderer, LineBatch};
use loaders::{load_texture};
use voxels::{Chunk, Chunks, chunk::CHUNK_D, chunk::CHUNK_W, chunk::CHUNK_H, chunk::CHUNK_VOL};
use graphics::mesh::Mesh;
use files::{read_binary_file, write_binary_file};
use lighting::Lighting;
use voxels::{Block, Blocks};

mod window;
mod graphics;
mod loaders;
mod voxels;
mod settings;
mod math;
mod files;
mod lighting;


const VERTICES: [f32; 8] = [
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

    let shader = load_shader("res/shaders/main.glslv","res/shaders/main.glslf").expect("Failed to load base shader");

    let crosshair_shader = load_shader("res/shaders/crosshair.glslv","res/shaders/crosshair.glslf").expect("Failed to load crosshair shader");

    let lines_shader = load_shader("res/shaders/lines.glslv", "res/shaders/lines.glslf").expect("Failed to load lines shader");

    let texture = load_texture("res/textures/atlas.png").expect("Failed to load texture");

    let mut blocks = Blocks::init();


    {
        // AIR
        let mut block = Block::new(0, 0);
        block.draw_group = 1;
        block.light_passing = true;
        blocks.blocks[block.id as usize] = Some(block.clone());

        // STONE
        block = Block::new(1, 2);
        blocks.blocks[block.id as usize] = Some(block.clone());

        // GRASS
        block = Block::new(2, 4);
        block.texture_faces[2] = 2;
        block.texture_faces[3] = 1;
        blocks.blocks[block.id as usize] = Some(block.clone());

        // LAMP
        block = Block::new(3, 3);
        block.emission[0] = 10;
        block.emission[1] = 0;
        block.emission[2] = 0;
        blocks.blocks[block.id as usize] = Some(block.clone());

        // GLASS
        block = Block::new(4, 5);
        block.draw_group = 2;
        block.light_passing = true;
        blocks.blocks[block.id as usize] = Some(block.clone());

        // GLASS
        block = Block::new(5, 6);
        blocks.blocks[block.id as usize] = Some(block.clone());
    }





    let mut chunks = Chunks::new(16, 16, 16);
    let mut meshes = Vec::with_capacity(chunks.volume);
    let mut renderer = VoxelRenderer::new(1024*1024*8);

    let mut linebatch = LineBatch::init(4096);

    for i in 0..chunks.volume {
        let mesh = renderer.render(&chunks.chunks[i], &vec![], &blocks);
        meshes.push(mesh);
    }


    window.clear_color(0.0, 0.0, 0.0, 1.0);

    window.setting_gl();

    let mut crosshair = Mesh::new(VERTICES.as_ptr(), 4, attrs.as_ptr());

    let mut camera = Camera::init(Vec3::new(20.0, 10.0, 20.0), 70.0_f32.to_radians());

    let mut last_time = window.glfw.get_time();
    let mut _delta:f64 = 0.0;

    let speed:f32 = 10.0f32;

    let mut cam_x = 0.0;
    let mut cam_y = 0.0;

    let mut choosen_block = 1;

    let mut buffer = vec![0u8; chunks.volume * CHUNK_VOL];
    let _result = read_binary_file("res/worlds/world.bin", &mut buffer);
    chunks.read(&buffer);


    let mut lighting = Lighting::init();

    lighting.on_world_loaded(&blocks, &mut chunks);

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

        if events.jpressed(K_1){
            choosen_block = 1;
        }

        if events.jpressed(K_2){
            choosen_block = 2;
        }

        if events.jpressed(K_3){
            choosen_block = 3;
        }

        if events.jpressed(K_4){
            choosen_block = 4;
        }

        if events.jpressed(K_5){
            choosen_block = 5;
        }

        if events.jpressed(K_6){
            choosen_block = 6;
        }

        if events.jpressed(K_7){
            choosen_block = 7;
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

        if events.jpressed(F1) {
            let mut buffer = vec![0u8; chunks.volume * CHUNK_VOL];
            chunks.write(&mut buffer);
            let _result = write_binary_file("res/worlds/world.bin", &buffer);
            println!("world saved in {} bytes", chunks.volume * CHUNK_VOL);
        }

        // if events.jpressed(F2) {
        //     let mut buffer = vec![0u8; chunks.volume * CHUNK_VOL];
        //     let _result = read_binary_file("res/worlds/world.bin", &mut buffer);
        //     chunks.read(&buffer);
        // }



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


        {
            let mut end = Vec3::ZERO;
            let mut norm = Vec3::ZERO;
            let mut iend = Vec3::ZERO;

            if
            let Some(vox) = chunks.ray_cast(
                camera.position,
                camera.front,
                10.0,
                &mut end,
                &mut norm,
                &mut iend
            )
            {
                linebatch.boxx(iend.x+0.5, iend.y+0.5, iend.z+0.5, 1.01, 1.01, 1.01, 1.0, 1.0, 1.0, 1.);

                if events.jclicked(LCM) {
                    let x = iend.x as isize;
                    let y = iend.y as isize;
                    let z = iend.z as isize;

                    chunks.set(x, y, z, 0);

                    lighting.on_block_set(x, y, z, choosen_block, &blocks, &mut chunks);
                }

                if events.jclicked(PCM) {
                    let x = (iend.x + norm.x) as isize;
                    let y = (iend.y + norm.y) as isize;
                    let z = (iend.z + norm.z) as isize;

                    chunks.set(x, y, z, choosen_block.into());

                    lighting.on_block_set(x, y, z, choosen_block, &blocks, &mut chunks);
                }
            }
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

            // if let Some(mesh) = meshes[i] {
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
                closes[index as usize] = Some(other.clone());
            }

            let mesh = renderer.render(chunk, &closes, &blocks);
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

        lines_shader.use_shader();
        lines_shader.uniform_matrix("preview", camera.get_projection(window.width() as f32, window.height() as f32));

        linebatch.line(
            0.0, 0.0, 0.0,
            0.0, 10.0, 0.0,

            1.0, 0.0, 0.0, 1.0
        );

        linebatch.line_width(2.0f32);

        linebatch.render();

        window.swap_buffers();
        window.poll_events();
        events.pull_events(&mut window);
    }

    window.terminate();
}