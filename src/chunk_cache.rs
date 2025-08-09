use crate::land_chunk::LandChunk;
use crate::map::Map;
use crate::statics_chunk::StaticsChunk;
use std::collections::{HashMap, VecDeque};

pub struct ChunkCache {
    capacity: u32,
    chunks: HashMap<u32, (LandChunk, StaticsChunk)>,
    queue: VecDeque<u32>,
}

impl ChunkCache {
    pub fn new(capacity: u32) -> Self {
        ChunkCache { capacity, chunks: HashMap::with_capacity((capacity + 1) as usize), queue: VecDeque::with_capacity((capacity + 1) as usize) }
    }

    fn block_id(x: u16, y: u16) -> u32 {
        (x * Map::MAX_DIM + y) as u32
    }

    ///
    /// Returned value will be something if we exceed capacity
    ///
    pub fn add(&mut self, tuple: (LandChunk, StaticsChunk)) -> Option<(LandChunk, StaticsChunk)> {
        let id = ChunkCache::block_id(tuple.0.x, tuple.0.y);
        if self.chunks.contains_key(&id) {
            return None
        };

        self.chunks.insert(id, tuple);
        self.queue.push_back(id);
        if self.queue.len() > self.capacity as usize {
            let id = self.queue.pop_front().unwrap();
            let chunk = self.chunks.remove(&id).unwrap();
            return Some(chunk)
        }
        None
    }

    pub fn contains(&self, x: u16, y: u16) -> bool {
        self.chunks.contains_key(&ChunkCache::block_id(x,y))
    }

    pub fn get(&self, x: u16, y: u16) -> &(LandChunk, StaticsChunk) {
        self.chunks.get(&ChunkCache::block_id(x, y)).expect("Used get on missing chunk!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let mut cache = ChunkCache::new(1);
        assert_eq!(false, cache.contains(0,0));

        let land = LandChunk::new(0,0, 0);
        let statics = StaticsChunk::new(0,0);
        cache.add((land, statics));

        assert_eq!(true, cache.contains(0,0));
        assert_eq!(false, cache.contains(1,0));
    }
}