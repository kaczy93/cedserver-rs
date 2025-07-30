mod map;
mod chunk_cache;

#[derive(Debug, Copy, Clone)]
pub struct LandTile {
    pub id: u16,
    pub x: u16,
    pub y: u16,
    pub z: i8,
}

impl LandTile {
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

#[derive(Debug)]
pub struct StaticTile {
    pub id: u16,
    pub x: u16,
    pub y: u16,
    pub z: i8,
    pub hue: u16,
}


impl StaticTile {
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

pub struct LandChunk {
    pub x: u16,
    pub y: u16,
    pub tiles: [[LandTile; 8]; 8]
}


impl LandChunk {
    pub fn new(x: u16, y: u16) -> LandChunk {
        LandChunk { x, y, tiles: [[LandTile::default(); 8]; 8] }
    }
}

pub struct StaticsChunk {
    pub x: u16,
    pub y: u16,
    pub tiles: [[Vec<StaticTile>; 8]; 8]
}

impl StaticsChunk {
    pub fn new(x: u16, y: u16) -> StaticsChunk {
        StaticsChunk {
            x,
            y,
            tiles: Default::default()
        }
    }
}
