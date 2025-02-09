use std::collections::VecDeque;
use crate::voxels::{chunk::{CHUNK_D, CHUNK_H, CHUNK_W}, chunks::Chunks };

#[derive(Clone, Copy)]
struct LightEntry {
    x: i32,
    y: i32,
    z: i32,
    light: u8,
}

pub struct LightSolver {
    add_queue: VecDeque<LightEntry>,
    rem_queue: VecDeque<LightEntry>,
    channel: i32,
}

impl LightSolver {
    pub fn new(channel: i32) -> Self {
        Self {
            add_queue: VecDeque::new(),
            rem_queue: VecDeque::new(),
            channel,
        }
    }

    pub fn add(&mut self, x: i32, y: i32, z: i32, emission: Option<i32>, chunks: &mut Chunks) {
        if let Some(emission) = emission {
            if emission <= 1 {
                return;
            }
            let entry = LightEntry {
                x,
                y,
                z,
                light: emission as u8,
            };
            self.add_queue.push_back(entry);

            let chunk = chunks.get_mut_chunk_by_voxel(
                entry.x as isize,
                entry.y as isize,
                entry.z as isize
            );

            if let Some(chunk) = chunk {
                chunk.modified = true;
                chunk.light_map.set(
                    (entry.x as usize) - (chunk.x as usize) * CHUNK_W as usize,
                    (entry.y as usize) - (chunk.y as usize) * CHUNK_H as usize,
                    (entry.z as usize) - (chunk.z as usize) * CHUNK_D as usize,
                    self.channel as usize,
                    entry.light
                );
            }
        } else {
            self.add(x as i32,y as i32,z as i32, Some(chunks.get_light(x as isize,y as isize,z as isize, self.channel as usize) as i32), chunks);
        }
    }

    pub fn _add_light(&mut self, x: isize, y: isize, z: isize, chunks: &mut Chunks) {
        self.add(
            x as i32,
            y as i32,
            z as i32,
            Some(chunks.get_light(x, y, z, self.channel as usize) as i32),
            chunks
        );
    }

    pub fn remove(&mut self, x: isize, y: isize, z: isize, chunks: &mut Chunks) {
        let chunk = chunks.get_mut_chunk_by_voxel(x, y, z);
        if let Some(chunk) = chunk {
            let light = chunk.light_map.get(
                (x - chunk.x * (CHUNK_W as isize)) as usize,
                (y - chunk.y * (CHUNK_H as isize)) as usize,
                (z - chunk.z * (CHUNK_D as isize)) as usize,
                self.channel as usize
            );
            if light == 0 {
                return;
            }
            let entry = LightEntry {
                x: x as i32,
                y: y as i32,
                z: z as i32,
                light,
            };
            self.rem_queue.push_back(entry);
            chunk.light_map.set(
                (x - chunk.x * (CHUNK_W as isize)) as usize,
                (y - chunk.y * (CHUNK_H as isize)) as usize,
                (z - chunk.z * (CHUNK_D as isize)) as usize,
                self.channel as usize,
                0
            );
        }
    }

    pub fn solve(&mut self, chunks: &mut Chunks) {
        let coords = [0, 0, 1, 0, 0, -1, 0, 1, 0, 0, -1, 0, 1, 0, 0, -1, 0, 0];

        while let Some(entry) = self.rem_queue.pop_front() {
            for i in 0..6 {
                let x = entry.x + coords[i * 3];
                let y = entry.y + coords[i * 3 + 1];
                let z = entry.z + coords[i * 3 + 2];
                let light = chunks.get_light(
                    x as isize,
                    y as isize,
                    z as isize,
                    self.channel as usize
                );
                if
                let Some(chunk) = chunks.get_mut_chunk_by_voxel(
                    x as isize,
                    y as isize,
                    z as isize
                )
                {
                    if light != 0 && light == entry.light - 1 {
                        let nentry = LightEntry { x, y, z, light };
                        self.rem_queue.push_back(nentry);
                        chunk.light_map.set(
                            (x as usize) - (chunk.x as usize) * CHUNK_W as usize,
                            (y as usize) - (chunk.y as usize) * CHUNK_H as usize,
                            (z as usize) - (chunk.z as usize) * CHUNK_D as usize,
                            self.channel as usize,
                            0
                        );
                        chunk.modified = true;
                    } else if light >= entry.light {
                        let nentry = LightEntry { x, y, z, light };
                        self.add_queue.push_back(nentry);
                    }
                }
            }
        }

        while let Some(entry) = self.add_queue.pop_front() {
            if entry.light <= 1 {
                continue;
            }
            for i in 0..6 {
                let x = entry.x + coords[i * 3];
                let y = entry.y + coords[i * 3 + 1];
                let z = entry.z + coords[i * 3 + 2];
                let light = chunks.get_light(
                    x as isize,
                    y as isize,
                    z as isize,
                    self.channel as usize
                );
                let v = chunks.get_voxel(x as isize, y as isize, z as isize).cloned();
                if
                let Some(chunk) = chunks.get_mut_chunk_by_voxel(
                    x as isize,
                    y as isize,
                    z as isize
                )
                {
                    if let Some(v) = v {
                        if v.id == 0 && light + 2 <= entry.light {
                            chunk.light_map.set(
                                (x as usize) - (chunk.x as usize) * CHUNK_W as usize,
                                (y as usize) - (chunk.y as usize) * CHUNK_H as usize,
                                (z as usize) - (chunk.z as usize) * CHUNK_D as usize,
                                self.channel as usize,
                                entry.light - 1
                            );
                            chunk.modified = true;
                            let nentry = LightEntry {
                                x,
                                y,
                                z,
                                light: entry.light - 1,
                            };
                            self.add_queue.push_back(nentry);
                        }
                    }
                }
            }
        }
    }
}