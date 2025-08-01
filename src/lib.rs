use std::ops::Index;
use bytes::Buf;

pub mod map;
pub mod server;
mod chunk_cache;
mod bytes_utf8;
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

    pub fn deserialize(mut data: &[u8]) -> MulIndex {
        let lookup = data.get_u32_le();
        let length = data.get_u32_le();
        let extra = data.get_u32_le();
        MulIndex { lookup, length, extra }
    }

    pub fn is_valid(&self) -> bool {
        self.length != u32::MAX
    }
}
