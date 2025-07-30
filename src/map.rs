use std::cmp;
use crate::{LandChunk, StaticsChunk, };
use crate::chunk_cache::ChunkCache;

pub struct Map{
    pub width: u16,
    pub height: u16,
    chunk_cache: ChunkCache
}

impl Map{
    pub const MAX_DIM: u16 = u16::MAX / 8;
    pub fn new(width: u16, height: u16) -> Map {
        let mut land_chunks: Vec<Vec<LandChunk>> = Vec::with_capacity(height as usize);
        land_chunks.fill_with(|| Vec::with_capacity(width as usize));

        let mut static_chunks: Vec<Vec<StaticsChunk>> = Vec::with_capacity(height as usize);
        static_chunks.fill_with(|| Vec::with_capacity(width as usize));

        Map {width, height, chunk_cache: ChunkCache::new(cmp::max(width, height) as u32)}
    }
    pub fn tile_width(&self) -> u16{
        self.width * 8
    }

    pub fn tile_height(&self) -> u16{
        self.height * 8
    }
}