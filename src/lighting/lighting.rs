use noise::NoiseFn;
use crate::lighting::LightSolver;
use crate::voxels::chunk::{CHUNK_D, CHUNK_H, CHUNK_VOL, CHUNK_W};
use crate::voxels::{Blocks, Chunks};

pub struct Lighting {
    solver_r: LightSolver,
    solver_g: LightSolver,
    solver_b: LightSolver,
    solver_s: LightSolver,
}


impl Lighting {
    pub fn init() -> Self {
        let solver_r = LightSolver::new(0);
        let solver_g = LightSolver::new(1);
        let solver_b = LightSolver::new(2);
        let solver_s = LightSolver::new(3);

        Lighting {
            solver_r,
            solver_g,
            solver_b,
            solver_s
        }
    }


    pub fn clear(&mut self, chunks: &mut Chunks) {
        for y in 0..chunks.h {
            for z in 0..chunks.d {
                for x in 0..chunks.w {
                    match chunks.get_mut_chunk(x as isize, y as isize, z as isize) {
                        Some(chunk) => {
                            for i in 0..CHUNK_VOL {
                                chunk.light_map.map[i] = 0;
                            }
                        }

                        None => {
                            // значит чанка нету
                        }
                    }
                }
            }
        }

    }


    pub fn on_world_loaded(&mut self, blocks: &Blocks, chunks: &mut Chunks) {
        let h = chunks.h;
        let d = chunks.d;
        let w = chunks.w;

        for y in 0..h * CHUNK_H as usize {
            for z in 0..d * CHUNK_D as usize {
                for x in 0..w * CHUNK_W as usize {
                    let vox = chunks.get_voxel(x as isize, y as isize, z as isize);
                    if let Some(vox) = vox {
                        if vox.id == 3 {
                            self.solver_r.add(x as i32, y as i32, z as i32, Some(15), chunks);
                            self.solver_g.add(x as i32, y as i32, z as i32, Some(15), chunks);
                            self.solver_b.add(x as i32, y as i32, z as i32, Some(15), chunks);
                        }
                    }
                }
            }
        }

        for z in 0..d * CHUNK_D as usize {
            for x in 0..w * CHUNK_W as usize {
                for y in 0..h * CHUNK_H as usize {
                    let vox = chunks.get_voxel(x as isize, y as isize, z as isize);
                    if let Some(vox) = vox {
                        if vox.id != 0 {
                            break;
                        }
                        let voxel = chunks.get_mut_chunk_by_voxel(
                            x as isize,
                            y as isize,
                            z as isize
                        );
                        if let Some(voxel) = voxel {
                            voxel.light_map.set_s(x % CHUNK_W as usize, y % CHUNK_H as usize, z % CHUNK_D as usize, 0xf);
                        }
                    }
                }
            }
        }

        for z in 0..d * CHUNK_D as usize {
            for x in 0..w * CHUNK_W as usize {
                for y in (0..=h * CHUNK_H as usize - 1).rev() {
                    let vox = chunks.get_voxel(x as isize, y as isize, z as isize);
                    if let Some(vox) = vox {
                        if vox.id != 0 {
                            break;
                        }

                        if
                        chunks.get_light((x as isize) - 1, y as isize, z as isize, 3) == 0 ||
                            chunks.get_light((x as isize) + 1, y as isize, z as isize, 3) == 0 ||
                            chunks.get_light(x as isize, (y as isize) - 1, z as isize, 3) == 0 ||
                            chunks.get_light(x as isize, (y as isize) + 1, z as isize, 3) == 0 ||
                            chunks.get_light(x as isize, y as isize, (z as isize) - 1, 3) == 0 ||
                            chunks.get_light(x as isize, y as isize, (z as isize) + 1, 3) == 0
                        {
                            self.solver_s.add(x as i32, y as i32, z as i32, None, chunks);
                        }
                        if
                        let Some(voxel) = chunks.get_mut_chunk_by_voxel(
                            x as isize,
                            y as isize,
                            z as isize
                        )
                        {
                            voxel.light_map.set_s(x % CHUNK_W as usize, y % CHUNK_H as usize, z % CHUNK_D as usize, 0xf);
                        }
                    }
                }
            }
        }

        self.solver_r.solve(&blocks, chunks);
        self.solver_g.solve(&blocks, chunks);
        self.solver_b.solve(&blocks, chunks);
        self.solver_s.solve(&blocks, chunks);
    }


    pub fn on_block_set(
        &mut self,
        x: isize,
        y: isize,
        z: isize,
        id: u8,
        blocks: &Blocks,
        chunks: &mut Chunks
    ) {
        if id == 0 {
            self.solver_r.remove(x, y, z, chunks);
            self.solver_g.remove(x, y, z, chunks);
            self.solver_b.remove(x, y, z, chunks);

            self.solver_r.solve(&blocks, chunks);
            self.solver_g.solve(&blocks, chunks);
            self.solver_b.solve(&blocks, chunks);

            if chunks.get_light(x, y + 1, z, 3) == 0xf {
                for i in (0..=y).rev() {
                    let voxel = chunks.get_voxel(x, i, z);
                    if let Some(voxel) = voxel {
                        if voxel.id != 0 {
                            break;
                        }
                        self.solver_s.add(x as i32, i as i32, z as i32, Some(0xf), chunks);
                    }
                }
            }

            let (x, y, z) = (x as i32, y as i32, z as i32);

            self.solver_r.add(x, y + 1, z, None, chunks);
            self.solver_g.add(x, y + 1, z, None, chunks);
            self.solver_b.add(x, y + 1, z, None, chunks);
            self.solver_s.add(x, y + 1, z, None, chunks);
            self.solver_r.add(x, y - 1, z, None, chunks);
            self.solver_g.add(x, y - 1, z, None, chunks);
            self.solver_b.add(x, y - 1, z, None, chunks);
            self.solver_s.add(x, y - 1, z, None, chunks);
            self.solver_r.add(x + 1, y, z, None, chunks);
            self.solver_g.add(x + 1, y, z, None, chunks);
            self.solver_b.add(x + 1, y, z, None, chunks);
            self.solver_s.add(x + 1, y, z, None, chunks);
            self.solver_r.add(x - 1, y, z, None, chunks);
            self.solver_g.add(x - 1, y, z, None, chunks);
            self.solver_b.add(x - 1, y, z, None, chunks);
            self.solver_s.add(x - 1, y, z, None, chunks);
            self.solver_r.add(x, y, z + 1, None, chunks);
            self.solver_g.add(x, y, z + 1, None, chunks);
            self.solver_b.add(x, y, z + 1, None, chunks);
            self.solver_s.add(x, y, z + 1, None, chunks);
            self.solver_r.add(x, y, z - 1, None, chunks);
            self.solver_g.add(x, y, z - 1, None, chunks);
            self.solver_b.add(x, y, z - 1, None, chunks);
            self.solver_s.add(x, y, z - 1, None, chunks);

            self.solver_r.solve(&blocks, chunks);
            self.solver_g.solve(&blocks, chunks);
            self.solver_b.solve(&blocks, chunks);
            self.solver_s.solve(&blocks, chunks);
        } else {
            self.solver_r.remove(x, y, z, chunks);
            self.solver_g.remove(x, y, z, chunks);
            self.solver_b.remove(x, y, z, chunks);
            self.solver_s.remove(x, y, z, chunks);

            for i in (0..=y - 1).rev() {
                self.solver_s.remove(x, i, z, chunks);
                if let Some(voxel) = chunks.get_voxel(x, i - 1, z) {
                    if i == 0 || voxel.id != 0 {
                        break;
                    }
                }
            }

            self.solver_r.solve(&blocks, chunks);
            self.solver_g.solve(&blocks, chunks);
            self.solver_b.solve(&blocks, chunks);
            self.solver_s.solve(&blocks, chunks);

            let block = blocks.get(id);

            if let Some(block) = block {
                if block.emission[0] != 0 || block.emission[0] != 0 || block.emission[0] != 0 {
                    let (x, y, z) = (x as i32, y as i32, z as i32);
                    self.solver_r.add(x, y, z, Some(block.emission[0] as i32), chunks);
                    self.solver_g.add(x, y, z, Some(block.emission[1] as i32), chunks);
                    self.solver_b.add(x, y, z, Some(block.emission[2] as i32), chunks);

                    self.solver_r.solve(&blocks, chunks);
                    self.solver_g.solve(&blocks, chunks);
                    self.solver_b.solve(&blocks, chunks);
                }
            }
        }
    }
}