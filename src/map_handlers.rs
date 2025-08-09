use crate::enums::AccessLevel;
use crate::net_state::NetState;
use crate::point::{PointU16, ReadPointU16};
use std::error::Error;
use bytes::BufMut;
use crate::land_chunk::{BufMutLandChunk, LandChunk};
use crate::statics_chunk::BufMutStaticsChunk;

impl NetState{
    pub(crate) fn on_blocks_request_packet(&self, mut data: &[u8]) -> Result<(), Box<dyn Error>> {
        println!("on_blocks_request_packet");//TODO: Only print when dev profile
        self.assert_access(AccessLevel::View)?;
        let blocks_count = data.len() / 4;
        for _i in 0..blocks_count {
            self.send_block_packet(data.get_point_u16())?;
        }
        //TODO: Zlib compress
        //TODO: Chunk subscriptions
        
        Ok(())
    }

    pub fn send_block_packet(&self, point: PointU16) -> Result<(), Box<dyn Error>> {
        let mut map = self.map_ref_mut();
        let (land_chunk, statics_chunk) = map.get_chunk(point.0, point.1);
        let data_len = 1 + 4 + 2 + 2 + LandChunk::BYTE_SIZE + 2 + statics_chunk.get_byte_size();
        let mut data: Vec<u8> = vec![0; data_len];
        let mut writer = &mut data[..];
        writer.put_u8(0x04); //Packet ID;
        writer.put_u32_le(data_len as u32);
        writer.put_u16_le(point.0);
        writer.put_u16_le(point.1);
        writer.put_land_chunk(land_chunk);
        writer.put_u16_le(statics_chunk.get_tile_count());
        //TODO: Sort static tiles
        writer.put_statics_chunk(statics_chunk);
        self.send(&data)?;
        Ok(())
    }
}