const BLOCK_COUNT: usize = 256;


#[derive(Clone)]
pub struct Block {
    pub id: u32,
    pub texture_faces: [i32; 6],
    pub emission: [u8; 3],
    pub draw_group: u8, // ??
    pub light_passing: bool,
}


pub struct Blocks {
    pub blocks: Vec<Option<Block>>,
}


impl Blocks {
    pub fn init() -> Self {
        Self { blocks: vec![None; BLOCK_COUNT]  }
    }

    pub fn get(&self, id: u8) -> Option<&Block> {
        self.blocks[id as usize].as_ref()
    }
}


impl Block {
    pub fn new(id: u32, texture: i32) -> Self {
        Self {
            id,
            texture_faces: [texture; 6],
            emission: [0; 3],
            draw_group: 0,
            light_passing: false,
        }
    }
}