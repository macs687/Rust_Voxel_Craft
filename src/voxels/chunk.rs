use noise::{NoiseFn, OpenSimplex};
use crate::voxels::voxel::Voxel;
use crate::lighting::light_map::LightMap;


pub const CHUNK_W: isize = 16; // X
pub const CHUNK_H: isize = 16; // Y
pub const CHUNK_D: isize = 16; // Z
pub const CHUNK_VOL: usize = (CHUNK_W * CHUNK_H * CHUNK_D) as usize;


#[derive(Debug, Clone)]
pub struct Chunk {
    pub voxels: [Voxel; CHUNK_VOL],
    pub x: isize,
    pub y: isize,
    pub z: isize,
    pub modified: bool,
    pub light_map: LightMap
}


impl Chunk {
    pub fn new(x_pos: isize, y_pos: isize, z_pos: isize) -> Self {
        let mut voxels = [Voxel {id: 0}; CHUNK_VOL];
        let perlin = OpenSimplex::new(1);

        for z in 0..CHUNK_D {
            for x in 0..CHUNK_W {
                let real_x = x + x_pos * CHUNK_W;
                let real_z = z + z_pos * CHUNK_D;
                //let height = perlin.get([(x as f64) * 0.05, (z as f64) * 0.05]);
                for y in 0..CHUNK_H {
                    let real_y = y + y_pos * CHUNK_H as isize;
                    let id = perlin.get([(real_x as f64) * 0.0125, (real_y as f64) * 0.0125, (real_z as f64) * 0.0125]) > 0.1;
                    let chunk_index = ((y * CHUNK_D + z) * CHUNK_W + x) as usize;
                    if real_y <= 2 {
                        voxels[chunk_index].id = 2;
                    } else {
                        voxels[chunk_index].id = id as u8;
                    }
                }
            }
        }

        Chunk { voxels, x: x_pos, y: y_pos, z:z_pos, modified: true, light_map: LightMap::new() }
    }
}