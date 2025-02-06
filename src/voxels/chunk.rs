use crate::voxels::voxel::Voxel;

pub const CHUNK_W: i32 = 16; // X
pub const CHUNK_H: i32 = 16; // Y
pub const CHUNK_D: i32 = 16; // Z
pub const CHUNK_VOL: usize = (CHUNK_W * CHUNK_H * CHUNK_D) as usize;


pub struct Chunk {
    pub voxels: Box<[Voxel; CHUNK_VOL]>,
}


impl Chunk {
    pub fn new() -> Self {
        let mut voxels = Box::new([Voxel { id: 0 }; CHUNK_VOL]);
        for y in 0..CHUNK_H {
            for z in 0..CHUNK_D {
                for x in 0..CHUNK_W {
                    let id = if y as f32 <= (f32::sin(x as f32 * 0.8) * 0.5 + 0.5) * 10.0 {
                        1
                    } else {
                        0
                    };
                    if y <= 2  {
                        voxels[((y * CHUNK_D + z) * CHUNK_W + x) as usize].id = 4;
                    }
                    if y >= 3 && y <= 5 {
                        voxels[((y * CHUNK_D + z) * CHUNK_W + x) as usize].id = 7;
                    }
                    else {
                        voxels[((y * CHUNK_D + z) * CHUNK_W + x) as usize].id = id;
                    }
                }
            }
        }
        Chunk { voxels }
    }
}