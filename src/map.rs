use std::cmp;
use std::fs;
use crate::{LandChunk, StaticsChunk, };
use crate::chunk_cache::ChunkCache;

pub struct MapConfig<'a>{
    pub index: u8,
    pub width: u16,
    pub height: u16,
    pub directory: &'a str
}

impl MapConfig<'_>{
    pub fn new(index: u8, width: u16, height: u16, directory: &str) -> MapConfig{
        MapConfig{index, width, height, directory}
    }
}

pub struct Map<'a>{
    map_config: MapConfig<'a>,
    chunk_cache: ChunkCache
}

impl Map<'_>{
    pub const MAX_DIM: u16 = u16::MAX / 8;
    pub fn build(map_config: MapConfig) -> Map {
        let mut land_chunks: Vec<Vec<LandChunk>> = Vec::with_capacity(map_config.height as usize);
        land_chunks.fill_with(|| Vec::with_capacity(map_config.width as usize));

        let mut static_chunks: Vec<Vec<StaticsChunk>> = Vec::with_capacity(map_config.height as usize);
        static_chunks.fill_with(|| Vec::with_capacity(map_config.width as usize));
        
        let cache_capcity = cmp::max(map_config.width, map_config.height) as u32;

        Map {map_config, chunk_cache: ChunkCache::new(cache_capcity)}
    }
    pub fn width(&self) -> u16{
        self.map_config.width
    }
    
    pub fn height(&self) -> u16{
        self.map_config.height
    }
    
    pub fn tile_width(&self) -> u16{
        self.map_config.width * 8
    }

    pub fn tile_height(&self) -> u16{
        self.map_config.height * 8
    }

    pub fn get_chunk(&mut self, x: u16, y: u16) -> &(LandChunk, StaticsChunk) {
        if self.chunk_cache.contains(x,y) {
            return self.chunk_cache.get(x,y);
        }
        let loaded = self.load_chunk(x, y);
        let unloaded = self.chunk_cache.add(loaded);
        if unloaded.is_some() {
            self.unload_chunk(unloaded.unwrap())
        }
        self.chunk_cache.get(x,y)
    }

    fn load_chunk(&self, x: u16, y: u16) -> (LandChunk, StaticsChunk){
        panic!("Not implemented")
    }

    fn unload_chunk(&mut self, tuple: (LandChunk, StaticsChunk)) {
        //TODO: Do stuff!
    }
}