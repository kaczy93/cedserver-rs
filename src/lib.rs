pub mod map;
pub mod server;
mod bytes_utf8;
mod chunk_cache;
mod chunks;
mod connection_handler;
mod mul_index;
mod net_state;
mod packets;
mod tiles;

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

#[cfg(test)]
mod tests{
    use crate::LoginState;

    #[test]
    pub fn enum_cast(){
        assert_eq!(0, LoginState::Ok as u8);
        assert_eq!(1, LoginState::InvalidUser as u8);
    }
}
