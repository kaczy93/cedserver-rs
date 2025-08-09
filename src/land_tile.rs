use bytes::{Buf, BufMut};

#[derive(Debug, Copy, Clone)]
pub struct LandTile {
    pub id: u16,
    pub x: u16,
    pub y: u16,
    pub z: i8,
}

impl LandTile {
    pub const BYTE_SIZE: usize = 3;
    pub fn default () -> LandTile {
        Self::new(0,0,0,0)
    }
    pub fn new(id: u16, x: u16, y: u16, z: i8) -> LandTile {
        LandTile { id, x, y, z }
    }

    pub fn local_x(&self) -> u8 {
        (self.x & 0x7) as u8
    }

    pub fn local_y(&self) -> u8 {
        (self.y & 0x7) as u8
    }
}

pub trait BufLandTile{
    fn get_land_tile(&mut self) -> LandTile;
}

impl BufLandTile for &[u8] {
    fn get_land_tile(&mut self) -> LandTile {
        let id = self.get_u16_le();
        let z = self.get_i8();
        LandTile {id, x: 0, y: 0, z}
    }
}

pub trait BufMutLandTile {
    fn put_land_tile(&mut self, tile: &LandTile);
}

impl BufMutLandTile for &mut [u8] {
    fn put_land_tile(&mut self, tile: &LandTile) {
        self.put_u16_le(tile.id);
        self.put_i8(tile.z);
    }
}