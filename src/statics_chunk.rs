use crate::static_tile::{BufMutStaticTile, BufStaticTile, StaticTile};
use bytes::Buf;

pub struct StaticsChunk {
    pub x: u16,
    pub y: u16,
    tile_matrix: [Vec<StaticTile>; 64],
    tiles_count: u16
}

impl StaticsChunk {
    pub fn get_byte_size(&self) -> usize {
        self.tiles_count as usize * StaticTile::BYTE_SIZE
    }

    pub fn get_tile_count(&self) -> u16 {
        self.tiles_count
    }

    pub fn new(x: u16, y: u16) -> StaticsChunk {
        StaticsChunk {
            x,
            y,
            tile_matrix: core::array::from_fn(|_| Vec::new()),
            tiles_count: 0
        }
    }

    pub fn read(chunk_x: u16, chunk_y: u16, mut data: &[u8]) -> StaticsChunk {
        let mut result = StaticsChunk::new(chunk_x, chunk_y);
        while data.remaining() > 0 {
            let mut static_tile = data.get_static_tile_mul();
            static_tile.x += chunk_x * 8;
            static_tile.y += chunk_y * 8;

            result.tile_matrix[(static_tile.local_y() * 8 + static_tile.local_x()) as usize].push(static_tile);
            result.tiles_count += 1;
        }
        //TODO: panic if there's still something to read
        result
    }

    pub fn get_tiles(&self, x: u16, y: u16) -> &Vec<StaticTile> {
        &self.tile_matrix[(y * 8 + x) as usize]
    }
}

pub trait BufMutStaticsChunk{
    fn put_statics_chunk(&mut self, statics_chunk: &StaticsChunk) -> ();
}

impl BufMutStaticsChunk for &mut [u8] {
    fn put_statics_chunk(&mut self, statics_chunk: &StaticsChunk) -> () {
        for tiles in &statics_chunk.tile_matrix {
            for tile in tiles {
                self.put_static_tile_mul(tile);
            }
        }
    }
}