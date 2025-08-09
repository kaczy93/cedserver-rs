use bytes::{Buf, BufMut};

#[derive(Debug)]
pub struct StaticTile {
    pub id: u16,
    pub x: u16,
    pub y: u16,
    pub z: i8,
    pub hue: u16,
}

impl StaticTile {
    pub const BYTE_SIZE: usize = 7;
    pub fn new(id: u16, x: u16, y: u16, z: i8, hue: u16) -> StaticTile {
        StaticTile { id, x, y, z, hue }
    }

    pub fn local_x(&self) -> u8 {
        (self.x & 0x7) as u8
    }

    pub fn local_y(&self) -> u8 {
        (self.y & 0x7) as u8
    }
}

pub trait BufStaticTile{
    fn get_static_tile_mul(&mut self) -> StaticTile;
    fn get_static_tile_net(&mut self) -> StaticTile;
}

impl BufStaticTile for &[u8]{
    fn get_static_tile_mul(&mut self) -> StaticTile {
        let id = self.get_u16_le();
        let local_x = self.get_u8();
        let local_y = self.get_u8();
        let z = self.get_i8();
        let hue = self.get_u16_le();
        
        StaticTile {id, x: local_x as u16, y: local_y as u16, z, hue}
    }

    fn get_static_tile_net(&mut self) -> StaticTile {
        let id = self.get_u16_le();
        let x = self.get_u16_le();
        let y = self.get_u16_le();
        let z = self.get_i8();
        let hue = self.get_u16_le();
        StaticTile {id, x, y, z, hue}
    }
}

pub trait BufMutStaticTile{
    fn put_static_tile_mul(&mut self, static_tile: &StaticTile);
    fn put_static_tile_net(&mut self, static_tile: &StaticTile);
}

impl BufMutStaticTile for &mut [u8]{
    fn put_static_tile_mul(&mut self, static_tile: &StaticTile) {
        self.put_u16_le(static_tile.id);
        self.put_u8(static_tile.local_x());
        self.put_u8(static_tile.local_y());
        self.put_i8(static_tile.z);
        self.put_u16_le(static_tile.hue);
    }

    fn put_static_tile_net(&mut self, static_tile: &StaticTile) {
        self.put_u16_le(static_tile.id);
        self.put_u16_le(static_tile.x);
        self.put_u16_le(static_tile.y);
        self.put_i8(static_tile.z);
        self.put_u16_le(static_tile.hue);
    }
}