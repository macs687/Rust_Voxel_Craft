use crate::voxels::chunk::{CHUNK_D, CHUNK_VOL, CHUNK_W};

#[derive(Debug, Clone)]
pub struct LightMap {
    map: Vec<u16>,
}

impl LightMap {
    pub fn new() -> Self {
        let mut map = vec![0; CHUNK_VOL];
        for value in &mut map {
            *value = 0x0000;
        }
        LightMap { map }
    }

    pub fn get(&self, x: usize, y: usize, z: usize, channel: usize) -> u8 {
        let index = y * CHUNK_D as usize * CHUNK_W as usize + z * CHUNK_W as usize + x;
        ((self.map[index] >> (channel << 2)) & 0xF) as u8
    }

    #[allow(unused)]
    pub fn get_r(&self, x: usize, y: usize, z: usize) -> u8 {
        let index = y * CHUNK_D as usize * CHUNK_W as usize + z * CHUNK_W as usize + x;
        (self.map[index] & 0xF) as u8
    }

    #[allow(unused)]
    pub fn get_g(&self, x: usize, y: usize, z: usize) -> u8 {
        let index = y * CHUNK_D as usize * CHUNK_W as usize + z * CHUNK_W as usize + x;
        ((self.map[index] >> 4) & 0xF) as u8
    }

    #[allow(unused)]
    pub fn get_b(&self, x: usize, y: usize, z: usize) -> u8 {
        let index = y * CHUNK_D as usize * CHUNK_W as usize + z * CHUNK_W as usize + x;
        ((self.map[index] >> 8) & 0xF) as u8
    }

    #[allow(unused)]
    pub fn get_s(&self, x: usize, y: usize, z: usize) -> u8 {
        let index = y * CHUNK_D as usize * CHUNK_W as usize + z * CHUNK_W as usize + x;
        ((self.map[index] >> 12) & 0xF) as u8
    }

    #[allow(unused)]
    pub fn set_r(&mut self, x: usize, y: usize, z: usize, value: u8) {
        let index = y * CHUNK_D as usize * CHUNK_W as usize + z * CHUNK_W as usize + x;
        self.map[index] = (self.map[index] & 0xFFF0) | value as u16;
    }

    #[allow(unused)]
    pub fn set_g(&mut self, x: usize, y: usize, z: usize, value: u8) {
        let index = y * CHUNK_D as usize * CHUNK_W as usize + z * CHUNK_W as usize + x;
        self.map[index] = (self.map[index] & 0xFF0F) | ((value as u16) << 4);
    }

    #[allow(unused)]
    pub fn set_b(&mut self, x: usize, y: usize, z: usize, value: u8) {
        let index = y * CHUNK_D as usize * CHUNK_W as usize + z * CHUNK_W as usize + x;
        self.map[index] = (self.map[index] & 0xF0FF) | ((value as u16) << 8);
    }

    pub fn set_s(&mut self, x: usize, y: usize, z: usize, value: u8) {
        let index = y * CHUNK_D as usize * CHUNK_W as usize + z * CHUNK_W as usize + x;
        self.map[index] = (self.map[index] & 0x0FFF) | ((value as u16) << 12);
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, channel: usize, value: u8) {
        let index = y * CHUNK_D as usize * CHUNK_W as usize + z * CHUNK_W as usize + x;
        self.map[index] = (self.map[index] & (0xFFFF & (!(0xF << (channel * 4))))) | ((value as u16) << (channel << 2));
    }
}