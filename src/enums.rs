pub enum AccessLevel{
    _None,
    View,
    Normal,
    _Developer,
    _Administrator = 255
}

pub enum LoginState{
    Ok,
    _InvalidUser,
    _InvalidPassword,
    _AlreadyLoggedIn,
    _NoAccess
}

#[cfg(test)]
mod tests{
    use crate::enums::LoginState;

    #[test]
    pub fn enum_cast(){
        assert_eq!(0, LoginState::Ok as u8);
        assert_eq!(1, LoginState::_InvalidUser as u8);
    }
}