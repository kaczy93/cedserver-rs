use std::ops::Index;
use crate::binary_reader::BinaryReader;

pub mod map;
mod chunk_cache;
mod binary_reader;

#[derive(Debug, Copy, Clone)]
pub struct LandTile {
    pub id: u16,
    pub x: u16,
    pub y: u16,
    pub z: i8,
}

impl LandTile {
    pub fn default () -> LandTile {
        Self::new(0,0,0,0)
    }
    pub fn new(id: u16, x: u16, y: u16, z: i8) -> LandTile {
        LandTile { id, x, y, z }
    }

    pub fn local_x(&self) -> u8 {
        (self.x & 0x7) as u8
    }

    pub fn local_y(&self) -> u8 {
        (self.y & 0x7) as u8
    }

    pub fn deserialize(x: u16, y: u16, reader: &mut BinaryReader) -> LandTile {
        let id = reader.read_u16();
        let z = reader.read_i8();
        LandTile {id, x, y, z}
    }
}

#[derive(Debug)]
pub struct StaticTile {
    pub id: u16,
    pub x: u16,
    pub y: u16,
    pub z: i8,
    pub hue: u16,
}


impl StaticTile {
    pub fn new(id: u16, x: u16, y: u16, z: i8, hue: u16) -> StaticTile {
        StaticTile { id, x, y, z, hue }
    }

    pub fn local_x(&self) -> u8 {
        (self.x & 0x7) as u8
    }

    pub fn local_y(&self) -> u8 {
        (self.y & 0x7) as u8
    }
}

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
}

impl Index<(usize, usize)> for LandChunk {
    type Output = LandTile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x,y) = index;
        &self.tiles[y * 8 + x]
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
}

impl Index<(usize, usize)> for StaticsChunk {
    type Output = Vec<StaticTile>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x,y) = index;
        &self.tiles[y * 8 + x]
    }
}

pub struct MulIndex {
    pub lookup: u32,
    pub length: u32,
    pub extra: u32
}

impl MulIndex {
    pub fn new(lookup: u32, length: u32, extra: u32) -> MulIndex {
        MulIndex { lookup, length, extra }
    }

    pub fn deserialize(mut reader: BinaryReader) -> MulIndex {
        let lookup = reader.read_u32();
        let length = reader.read_u32();
        let extra = reader.read_u32();
        MulIndex { lookup, length, extra }
    }
}
