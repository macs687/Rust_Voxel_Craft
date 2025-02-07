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
        for y in 0..CHUNK_H {
            for z in 0..CHUNK_D {
                for x in 0..CHUNK_W {
                    let real_x = x + x_pos * CHUNK_W;
                    let real_y = y + y_pos * CHUNK_H;
                    let real_z = z + z_pos * CHUNK_W;
                    let id = if y as f32 <= ((x as f32 * 0.8) * 0.5 + 0.5) * 10.0 {
                        1
                    } else {
                        0
                    };
                    if y <= 2  {
                        voxels[((y * CHUNK_D + z) * CHUNK_W + x) as usize].id = 4;
                    }
                    if y >= 3 && y <= 5 {
                        voxels[((y * CHUNK_D + z) * CHUNK_W + x) as usize].id = 6;
                    }
                    else {
                        voxels[((y * CHUNK_D + z) * CHUNK_W + x) as usize].id = id;
                    }
                }
            }
        }
        Chunk { voxels, x: x_pos, y: y_pos, z:z_pos, modified: true }
    }
}