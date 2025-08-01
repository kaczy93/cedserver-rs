use fs::File;
use path::Path;
use std::cmp;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path;
use crate::MulIndex;
use crate::chunk_cache::ChunkCache;
use crate::chunks::{LandChunk, StaticsChunk};

pub struct MapConfig<'a>{
    pub map_index: u8,
    pub width: u16,
    pub height: u16,
    pub directory: &'a str
}

impl MapConfig<'_>{
    pub fn new(map_index: u8, width: u16, height: u16, directory: &str) -> MapConfig{
        MapConfig{ map_index, width, height, directory}
    }
}

pub struct Map<'a>{
    map_config: MapConfig<'a>,
    map_file: File,
    staidx_file: File,
    statics_file: File,
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

        //TODO: UOP
        let base_path = Path::new(map_config.directory);

        let map_path = base_path.join(format!("map{}.mul", map_config.map_index));
        let map_file = File::open(map_path).expect("Unable to open map file");

        let staidx_path = base_path.join(format!("staidx{}.mul", map_config.map_index));
        let staidx_file = File::open(staidx_path).expect("Unable to open staidx file");

        let statics_path = base_path.join(format!("statics{}.mul", map_config.map_index));
        let statics_file = File::open(statics_path).expect("Unable to open statics file");

        Map {map_config, map_file, staidx_file, statics_file, chunk_cache: ChunkCache::new(cache_capcity)}
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

    fn block_num(&self, x: u16, y: u16) -> u64 {
        x as u64 * self.height() as u64 + y as u64
    }

    fn map_offset(&self, x: u16, y: u16) -> u64 {
        self.block_num(x, y) * 196
    }

    fn staidx_offset(&self, x: u16, y: u16) -> u64 {
        self.block_num(x, y) * 12
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

    fn load_chunk(&mut self, x: u16, y: u16) -> (LandChunk, StaticsChunk){
        let mut land_buf: [u8; 196] = [0; 196];
        self.map_file.seek(SeekFrom::Start(self.map_offset(x, y))).expect("Unable to seek map file");
        self.map_file.read(&mut land_buf).expect("Error reading map file");
        let land_chunk = LandChunk::deserialize(x, y, &land_buf);

        let mut staidx_buf: [u8; 12] = [0; 12];
        self.staidx_file.seek(SeekFrom::Start(self.staidx_offset(x,y))).expect("Unable to seek staidx file");
        self.staidx_file.read(&mut staidx_buf).expect("Error reading staidx file");
        let statics_index = MulIndex::deserialize(&staidx_buf);

        let statick_chunk = if !statics_index.is_valid() {
            StaticsChunk::new(x,y)
        } else {
            let mut statics_buf: Vec<u8> = vec![0; statics_index.length as usize];
            self.statics_file.seek(SeekFrom::Start(statics_index.lookup as u64)).expect("Unable to seek staidx file");
            self.statics_file.read(&mut statics_buf).expect("Error reading staidx file");
            StaticsChunk::deserialize(x,y, &statics_buf)
        };
        (land_chunk, statick_chunk)

    }

    fn unload_chunk(&mut self, tuple: (LandChunk, StaticsChunk)) {
        //TODO: Do stuff!
    }
}

#[cfg(test)]
mod tests{
    use tempfile::tempfile;
    use crate::chunk_cache::ChunkCache;
    use crate::map::{Map, MapConfig};

    impl Map<'_> {
        fn dummy(width: u16, height: u16) -> Map<'static> {
            let map_file = tempfile().unwrap();
            let staidx_file= tempfile().unwrap();
            let statics_file = tempfile().unwrap();
            Map { map_config: MapConfig{ map_index: 0, width, height, directory: "dummy"},
                map_file,
                staidx_file,
                statics_file,
                chunk_cache: ChunkCache::new(64) }
        }
    }

    #[test]
    fn test_block_num(){
        let map = Map::dummy(Map::MAX_DIM, Map::MAX_DIM);
        assert_eq!(67_100_672, map.block_num(u16::MAX / 8,u16::MAX / 8))
    }
}
