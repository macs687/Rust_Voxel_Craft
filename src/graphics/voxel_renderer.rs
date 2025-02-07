use crate::voxels::{Chunk, voxel::Voxel, chunk::CHUNK_D, chunk::CHUNK_H, chunk::CHUNK_W };

use super::mesh::Mesh;

const VERTEX_SIZE: usize = 6;

fn cdiv(x: isize, a: isize) -> isize {
    if x < 0 {
        x / a - 1
    } else {
        x / a
    }
}

fn local_neg(x: isize, size: isize) -> isize {
    if x < 0 {
        size + x
    } else {
        x
    }
}

fn local(x: isize, size: isize) -> isize {
    if x >= size {
        x - size
    } else {
        local_neg(x, size)
    }
}

fn is_chunk(x: isize, y: isize, z: isize, chunks: &[Option<Chunk>]) -> bool {
    get_chunk(x, y, z, chunks).is_some()
}

fn get_chunk(x: isize, y: isize, z: isize, chunks: &[Option<Chunk>]) -> Option<&Chunk> {
    let index = ((cdiv(y, CHUNK_H as isize) + 1) * 3 + cdiv(z, CHUNK_D as isize) + 1) * 3 + cdiv(x, CHUNK_W as isize) + 1;
    if index >= 0 && index < chunks.len() as isize {
        chunks[index as usize].as_ref()
    } else {
        None
    }
}

fn voxel(x: isize, y: isize, z: isize, chunks: &[Option<Chunk>]) -> Option<&Voxel> {
    if let Some(chunk) = get_chunk(x, y, z, chunks) {
        let lx = local(x, CHUNK_W as isize) as usize;
        let ly = local(y, CHUNK_H as isize) as usize;
        let lz = local(z, CHUNK_D as isize) as usize;
        Some(&chunk.voxels[(ly * CHUNK_D as usize + lz) * CHUNK_W as usize + lx])
    } else {
        None
    }
}

fn is_blocked(x: isize, y: isize, z: isize, chunks: &[Option<Chunk>]) -> bool {
    !is_chunk(x, y, z, chunks) || voxel(x, y, z, chunks).map_or(false, |voxel| voxel.id != 0)
}

fn vertex(
    buffer: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
    u: f32,
    v: f32,
    l: f32
) {
    buffer.push(x);
    buffer.push(y);
    buffer.push(z);
    buffer.push(u);
    buffer.push(v);
    buffer.push(l);
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

    pub fn render(&mut self, chunk: &Chunk, chunks: &Vec<Option<Chunk>>) -> Mesh {
        self.buffer.clear();

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

                    if !is_blocked(x , y  + 1, z , &chunks) {
                        l = 1.0;
                        vertex(&mut self.buffer, x as f32 - 0.5, y as f32 + 0.5, z as f32 - 0.5, u + uvsize, v, l);
                        vertex(&mut self.buffer, x as f32 - 0.5, y as f32 + 0.5, z as f32 + 0.5, u + uvsize, v + uvsize, l);
                        vertex(&mut self.buffer, x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5, u, v + uvsize, l);

                        vertex(&mut self.buffer, x as f32 - 0.5, y as f32 + 0.5, z as f32 - 0.5, u + uvsize, v, l);
                        vertex(&mut self.buffer, x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5, u, v + uvsize, l);
                        vertex(&mut self.buffer, x as f32 + 0.5, y as f32 + 0.5, z as f32 - 0.5, u, v, l);
                    }
                    if !is_blocked(x,y -1,z, &chunks){
                        l = 0.75;
                        vertex(&mut self.buffer, x as f32 - 0.5, y as f32 - 0.5, z as f32 - 0.5, u, v, l);
                        vertex(&mut self.buffer, x as f32 + 0.5, y as f32 - 0.5, z as f32 + 0.5, u + uvsize, v + uvsize, l);
                        vertex(&mut self.buffer, x as f32 - 0.5, y as f32 - 0.5, z as f32 + 0.5, u, v + uvsize, l);

                        vertex(&mut self.buffer,x as f32 - 0.5, y as f32 - 0.5, z as f32 - 0.5, u, v, l);
                        vertex(&mut self.buffer,x as f32 + 0.5, y as f32 - 0.5, z as f32 - 0.5, u + uvsize, v, l);
                        vertex(&mut self.buffer,x as f32 + 0.5, y as f32 - 0.5, z as f32 + 0.5, u+uvsize, v + uvsize, l);
                    }

                    if !is_blocked(x+1,y, z, &chunks){
                        l = 0.95;
                        vertex(&mut self.buffer,x as f32 + 0.5, y as f32 - 0.5, z as f32 - 0.5, u+uvsize, v, l);
                        vertex(&mut self.buffer,x as f32 + 0.5, y as f32 + 0.5, z as f32 - 0.5, u + uvsize, v + uvsize, l);
                        vertex(&mut self.buffer,x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5, u, v + uvsize, l);

                        vertex(&mut self.buffer,x as f32 + 0.5, y as f32 - 0.5, z as f32 - 0.5, u+uvsize, v, l);
                        vertex(&mut self.buffer,x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5, u, v + uvsize, l);
                        vertex(&mut self.buffer,x as f32 + 0.5, y as f32 - 0.5, z as f32 + 0.5, u, v, l);
                    }
                    if !is_blocked(x-1,y,z, &chunks){
                        l = 0.85;
                        vertex(&mut self.buffer,x as f32 - 0.5, y as f32 - 0.5, z as f32 - 0.5, u, v, l);
                        vertex(&mut self.buffer,x as f32 - 0.5, y as f32 + 0.5, z as f32 + 0.5, u+uvsize, v + uvsize, l);
                        vertex(&mut self.buffer,x as f32 - 0.5, y as f32 + 0.5, z as f32 - 0.5, u, v+uvsize, l);

                        vertex(&mut self.buffer,x as f32 - 0.5, y as f32 - 0.5, z as f32 - 0.5, u, v, l);
                        vertex(&mut self.buffer,x as f32 - 0.5, y as f32 - 0.5, z as f32 + 0.5, u+uvsize, v, l);
                        vertex(&mut self.buffer,x as f32 - 0.5, y as f32 + 0.5, z as f32 + 0.5, u+uvsize, v+uvsize, l);
                    }

                    if !is_blocked(x,y,z+1, &chunks){
                        l = 0.9;
                        vertex(&mut self.buffer,x as f32 - 0.5, y as f32 - 0.5, z as f32 + 0.5, u, v, l);
                        vertex(&mut self.buffer,x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5, u+uvsize, v +uvsize, l);
                        vertex(&mut self.buffer,x as f32 - 0.5, y as f32 + 0.5, z as f32 + 0.5, u, v+uvsize, l);

                        vertex(&mut self.buffer,x as f32 - 0.5, y as f32 - 0.5, z as f32 + 0.5, u, v, l);
                        vertex(&mut self.buffer,x as f32 + 0.5, y as f32 - 0.5, z as f32 + 0.5, u+uvsize, v, l);
                        vertex(&mut self.buffer,x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5, u+uvsize, v+uvsize, l);
                    }
                    if !is_blocked(x,y,z-1,&chunks){
                        l = 0.8;
                        vertex(&mut self.buffer,x as f32 - 0.5, y as f32 - 0.5, z as f32 - 0.5, u+uvsize, v, l);
                        vertex(&mut self.buffer,x as f32 - 0.5, y as f32 + 0.5, z as f32 - 0.5, u+uvsize, v+uvsize, l);
                        vertex(&mut self.buffer,x as f32 + 0.5, y as f32 + 0.5, z as f32 - 0.5, u, v+uvsize, l);

                        vertex(&mut self.buffer,x as f32 - 0.5, y as f32 - 0.5, z as f32 - 0.5, u+uvsize, v, l);
                        vertex(&mut self.buffer,x as f32 + 0.5, y as f32 + 0.5, z as f32 - 0.5, u, v+uvsize, l);
                        vertex(&mut self.buffer,x as f32 + 0.5, y as f32 - 0.5, z as f32 - 0.5, u, v, l);
                    }
                }
            }
        }

        Mesh::new(self.buffer.as_ptr(), self.buffer.len() / VERTEX_SIZE, [3, 2, 1, 0].as_ptr())
    }
}