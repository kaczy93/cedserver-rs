use std::ops::Index;
use crate::binary_reader::BinaryReader;

pub mod map;
mod chunk_cache;
mod binary_reader;
mod tiles;
mod chunks;


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

    pub fn is_valid(&self) -> bool {
        self.length != u32::MAX
    }
}
