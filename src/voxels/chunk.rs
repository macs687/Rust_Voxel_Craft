use noise::{NoiseFn, OpenSimplex};
use crate::voxels::voxel::Voxel;

pub const CHUNK_W: isize = 16; // X
pub const CHUNK_H: isize = 16; // Y
pub const CHUNK_D: isize = 16; // Z
pub const CHUNK_VOL: usize = (CHUNK_W * CHUNK_H * CHUNK_D) as usize;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub voxels: Box<[Voxel; CHUNK_VOL]>,
    pub x: isize,
    pub y: isize,
    pub z: isize,
    pub modified: bool
}


impl Chunk {
    pub fn new(x_pos: isize, y_pos: isize, z_pos: isize) -> Self {
        let mut voxels = Box::new([Voxel { id: 0 }; CHUNK_VOL]);
        let perlin = OpenSimplex::new(1);

        for z in 0..CHUNK_D {
            for x in 0..CHUNK_W {
                let _real_x = x + x_pos * CHUNK_W;
                let _real_z = z + z_pos * CHUNK_D;
                let height = perlin.get([(x as f64) * 0.05, (z as f64) * 0.05]);
                for y in 0..CHUNK_H {
                    let real_y = y + y_pos * CHUNK_H;
                    let id = if (real_y as f32) <= ((height as f32) * 0.5 + 0.5 ) * 5.0  { 6 } else { 0 };
                    if real_y <= 2 {
                        voxels[((y * CHUNK_D + z) * CHUNK_W + x) as usize].id = 2;
                    } else {
                        voxels[((y * CHUNK_D + z) * CHUNK_W + x) as usize].id = id;
                    }
                }
            }
        }

        Chunk { voxels, x: x_pos, y: y_pos, z:z_pos, modified: true }
    }
}