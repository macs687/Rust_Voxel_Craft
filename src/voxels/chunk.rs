use crate::voxels::voxel::Voxel;

pub const CHUNK_W: i32 = 16; // X
pub const CHUNK_H: i32 = 16; // Y
pub const CHUNK_D: i32 = 16; // Z
pub const CHUNK_VOL: usize = (CHUNK_W * CHUNK_H * CHUNK_D) as usize;


pub struct Chunk {
    pub voxels: [Voxel; CHUNK_VOL],
}


impl Chunk {
    pub fn new(){
        let mut voxels = [Voxel {id: 0}; CHUNK_VOL];
        for y in 0..CHUNK_H {
            for z in 0..CHUNK_D {
                for x in 0..CHUNK_W {
                    voxels[((y * CHUNK_W + z) * CHUNK_W + x) as usize].id <= 5;
                }
            }


        }
    }

}