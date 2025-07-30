use std::collections::{HashMap, VecDeque};
use crate::{LandChunk, StaticsChunk};
use crate::map::Map;

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
    pub fn add(&mut self, land_chunk: LandChunk, statics_chunk: StaticsChunk) -> Option<(LandChunk, StaticsChunk)> {
        let id = ChunkCache::block_id(land_chunk.x, land_chunk.y);
        if self.chunks.contains_key(&id) {
            return None
        };

        self.chunks.insert(id, (land_chunk, statics_chunk));
        self.queue.push_back(id);
        if self.queue.len() > self.capacity as usize {
            let id = self.queue.pop_front().unwrap();
            let chunk = self.chunks.remove(&id).unwrap();
            return Some(chunk)
        }
        None
    }

    pub fn get(&self, x: u16, y: u16) -> Option<&(LandChunk, StaticsChunk)> {
        self.chunks.get(&ChunkCache::block_id(x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let mut cache = ChunkCache::new(1);
        assert!(cache.get(0,0).is_none());

        let land = LandChunk::new(0,0);
        let statics = StaticsChunk::new(0,0);
        cache.add(land, statics);

        assert!(cache.get(0,0).is_some());
        assert!(cache.get(1,0).is_none());
    }
}