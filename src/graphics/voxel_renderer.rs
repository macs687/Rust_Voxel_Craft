use crate::voxels::{ Chunk, voxel::Voxel, chunk::CHUNK_D, chunk::CHUNK_H, chunk::CHUNK_W };

use super::mesh::Mesh;

const VERTEX_SIZE: usize = 6;

fn is_in(x: isize, y: isize, z: isize) -> bool {
    x >= 0 &&
        x < (CHUNK_W as isize) &&
        y >= 0 &&
        y < (CHUNK_H as isize) &&
        z >= 0 &&
        z < (CHUNK_D as isize)
}

fn voxel(x: isize, y: isize, z: isize, chunk: &Chunk) -> &Voxel {
    &chunk.voxels[((y * (CHUNK_D as isize) + z) * (CHUNK_W as isize) + x) as usize]
}

fn is_blocked(x: isize, y: isize, z: isize, chunk: &Chunk) -> bool {
    is_in(x, y, z) && voxel(x, y, z, chunk).id != 0
}

fn vertex(
    buffer: &mut Vec<f32>,
    index: &mut usize,
    x: f32,
    y: f32,
    z: f32,
    u: f32,
    v: f32,
    l: f32
) {
    buffer.push(x - 0.5);
    buffer.push(y - 0.5);
    buffer.push(z - 0.5);
    buffer.push(u);
    buffer.push(v);
    buffer.push(l);
    *index += VERTEX_SIZE;
}

pub struct VoxelRenderer {
    buffer: Vec<f32>,
}

impl VoxelRenderer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity * VERTEX_SIZE * 6),
        }
    }

    pub fn render(&mut self, chunk: &Chunk) -> Mesh {
        self.buffer.clear();
        let mut index = 0;

        for y in 0_..CHUNK_H {
            for z in 0_..CHUNK_D {
                for x in 0..CHUNK_W {
                    let voxel = &chunk.voxels[(y * CHUNK_D * CHUNK_W + z * CHUNK_W + x) as usize];
                    let id = voxel.id;

                    if id == 0 {
                        continue;
                    }

                    let uvsize = 1.0 / 16.0;
                    let u = ((id % 16) as f32) * uvsize;
                    let v = 1.0 - ((1 + id / 16) as f32) * uvsize;

                    let mut l;
                    let (x, y, z) = (x as isize, y as isize, z as isize);

                    if !is_blocked(x , y  + 1, z , chunk) {
                        l = 1.0;
                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 + 0.5, z as f32 - 0.5, u + uvsize, v, l);
                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 + 0.5, z as f32 + 0.5, u + uvsize, v + uvsize, l);
                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5, u, v + uvsize, l);

                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 + 0.5, z as f32 - 0.5, u + uvsize, v, l);
                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5, u, v + uvsize, l);
                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 + 0.5, z as f32 - 0.5, u, v, l);
                    }
                    if !is_blocked(x,y -1,z, chunk){
                        l = 0.75;
                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 - 0.5, z as f32 - 0.5, u, v, l);
                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 - 0.5, z as f32 + 0.5, u + uvsize, v + uvsize, l);
                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 - 0.5, z as f32 + 0.5, u, v + uvsize, l);

                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 - 0.5, z as f32 - 0.5, u, v, l);
                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 - 0.5, z as f32 - 0.5, u + uvsize, v, l);
                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 - 0.5, z as f32 + 0.5, u+uvsize, v + uvsize, l);
                    }

                    if !is_blocked(x+1,y, z, chunk){
                        l = 0.95;
                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 - 0.5, z as f32 - 0.5, u+uvsize, v, l);
                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 + 0.5, z as f32 - 0.5, u + uvsize, v + uvsize, l);
                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5, u, v + uvsize, l);

                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 - 0.5, z as f32 - 0.5, u+uvsize, v, l);
                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5, u, v + uvsize, l);
                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 - 0.5, z as f32 + 0.5, u, v, l);
                    }
                    if !is_blocked(x-1,y,z, chunk){
                        l = 0.85;
                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 - 0.5, z as f32 - 0.5, u, v, l);
                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 + 0.5, z as f32 + 0.5, u+uvsize, v + uvsize, l);
                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 + 0.5, z as f32 - 0.5, u, v+uvsize, l);

                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 - 0.5, z as f32 - 0.5, u, v, l);
                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 - 0.5, z as f32 + 0.5, u+uvsize, v, l);
                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 + 0.5, z as f32 + 0.5, u+uvsize, v+uvsize, l);
                    }

                    if !is_blocked(x,y,z+1, chunk){
                        l = 0.9;
                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 - 0.5, z as f32 + 0.5, u, v, l);
                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5, u+uvsize, v +uvsize, l);
                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 + 0.5, z as f32 + 0.5, u, v+uvsize, l);

                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 - 0.5, z as f32 + 0.5, u, v, l);
                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 - 0.5, z as f32 + 0.5, u+uvsize, v, l);
                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5, u+uvsize, v+uvsize, l);
                    }
                    if !is_blocked(x,y,z-1,chunk){
                        l = 0.8;
                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 - 0.5, z as f32 - 0.5, u+uvsize, v, l);
                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 + 0.5, z as f32 - 0.5, u+uvsize, v+uvsize, l);
                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 + 0.5, z as f32 - 0.5, u, v+uvsize, l);

                        vertex(&mut self.buffer, &mut index, x as f32 - 0.5, y as f32 - 0.5, z as f32 - 0.5, u+uvsize, v, l);
                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 + 0.5, z as f32 - 0.5, u, v+uvsize, l);
                        vertex(&mut self.buffer, &mut index, x as f32 + 0.5, y as f32 - 0.5, z as f32 - 0.5, u, v, l);
                    }
                }
            }
        }

        Mesh::new(self.buffer.as_ptr(), index / VERTEX_SIZE, [3, 2, 1, 0].as_ptr())
    }
}