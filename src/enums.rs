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
    use crate::enums::LoginState;

    #[test]
    pub fn enum_cast(){
        assert_eq!(0, LoginState::Ok as u8);
        assert_eq!(1, LoginState::InvalidUser as u8);
    }
}