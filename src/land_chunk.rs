use crate::land_tile::{BufLandTile, BufMutLandTile, LandTile};
use bytes::{Buf, BufMut};

pub struct LandChunk {
    pub x: u16,
    pub y: u16,
    header: u32,
    tiles: [LandTile; 64]
}

impl LandChunk {
    pub const BYTE_SIZE: usize = 4 + 64 * LandTile::BYTE_SIZE;
    pub fn new(x: u16, y: u16, header: u32) -> LandChunk {
        LandChunk {x, y, header, tiles: [LandTile::default(); 64] }
    }

    pub fn read(chunk_x: u16, chunk_y: u16, mut data: &[u8]) -> LandChunk {
        let header = data.get_u32_le();
        let mut result = LandChunk::new(chunk_x, chunk_y, header);
        for local_y  in 0..8u16 {
            for local_x in 0..8u16 {
                let mut tile = data.get_land_tile();
                tile.x = chunk_x * 8 + local_x;
                tile.y = chunk_y * 8 + local_y;
                result.tiles[(local_y * 8 + local_x) as usize] = tile;
            }
        }
        result
    }
    
    pub fn get_tile(&self, x: u16, y: u16) -> &LandTile {
        &self.tiles[(y * 8 + x) as usize]
    }
}

pub trait BufMutLandChunk{
    fn put_land_chunk(&mut self, chunk: &LandChunk);
}

impl BufMutLandChunk for &mut [u8] {
    fn put_land_chunk(&mut self, land_chunk: &LandChunk) {
        self.put_u32_le(land_chunk.header);
        for land_tile in &land_chunk.tiles {
            self.put_land_tile(land_tile);   
        }
    }
}
