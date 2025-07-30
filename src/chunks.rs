use crate::binary_reader::BinaryReader;
use crate::tiles::{LandTile, StaticTile};

pub struct LandChunk {
    pub x: u16,
    pub y: u16,
    tiles: [LandTile; 64]
}

impl LandChunk {
    pub fn new(x: u16, y: u16) -> LandChunk {
        LandChunk { x, y, tiles: [LandTile::default(); 64] }
    }

    pub fn deserialize(chunk_x: u16, chunk_y: u16, mut reader: BinaryReader) -> LandChunk {
        let _header = reader.read_u32();
        let mut result = LandChunk::new(chunk_x, chunk_y);
        for local_y  in 0..8u16 {
            for local_x in 0..8u16 {
                result.tiles[(local_y * 8 + local_x) as usize] = LandTile::deserialize(chunk_x * 8 + local_x, chunk_y * 8 + local_y, &mut reader)
            }
        }
        result
    }
    
    pub fn get_tile(&self, x: u16, y: u16) -> &LandTile {
        &self.tiles[(y * 8 + x) as usize]
    }
}

pub struct StaticsChunk {
    pub x: u16,
    pub y: u16,
    tiles: [Vec<StaticTile>; 64]
}

impl StaticsChunk {
    pub fn new(x: u16, y: u16) -> StaticsChunk {
        StaticsChunk {
            x,
            y,
            tiles: core::array::from_fn(|_| Vec::new())
        }
    }

    pub fn deserialize(chunk_x: u16, chunk_y: u16, mut reader: BinaryReader) -> StaticsChunk {
        let mut result = StaticsChunk::new(chunk_x, chunk_y);
        while reader.has_next() {
            let id = reader.read_u16();
            let local_x = reader.read_u8();
            let local_y = reader.read_u8();
            let z = reader.read_i8();
            let hue = reader.read_u16();

            let x = chunk_x * 8 + local_x as u16;
            let y = chunk_y * 8 + local_y as u16;

            result.tiles[(local_y * 8 + local_x) as usize].push(StaticTile::new(id, x, y, z, hue));
        }
        result
    }
    
    pub fn get_tiles(&self, x: u16, y: u16) -> &Vec<StaticTile> {
        &self.tiles[(y * 8 + x) as usize]
    }
}