use std::ops::Index;
use bytes::Buf;

pub mod map;
pub mod server;
mod chunk_cache;
mod bytes_utf8;
mod tiles;
mod chunks;
mod connection_handler;
mod net_state;
mod packets;

pub enum AccessLevel{
    None,
    View,
    Normal,
    Developer,
    Administrator = 255
}

pub enum LoginState{
    Ok,
    InvalidUser,
    InvalidPassword,
    AlreadyLoggedIn,
    NoAccess
}

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

#[cfg(test)]
mod tests{
    use crate::LoginState;

    #[test]
    pub fn enum_cast(){
        assert_eq!(0, LoginState::Ok as u8);
        assert_eq!(1, LoginState::InvalidUser as u8);
    }
}
