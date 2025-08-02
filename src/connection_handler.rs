use bytes::Buf;
use crate::bytes_utf8::BytesUtf8;
use crate::LoginState;
use crate::net_state::NetState;

impl NetState {
    pub(crate) fn on_connection_packet(&self, mut data: &[u8]) -> () {
        let cmd_id = data.get_u8();
        match cmd_id {
            0x03 => self.on_login_request(data),
            0x05 => self.on_quit(data),
            _ => panic!("Unknown connection handler command {cmd_id}")
        }
    }

    fn on_login_request(&self, mut data: &[u8]) -> () {
        let username = data.get_str_nul();
        println!("User: {username}");
        let password = data.get_str_nul();
        println!("Password: {password}");
        //TODO: Check if valid
        self.send_login_response(LoginState::Ok)
    }

    fn on_quit(&self, mut data: &[u8]) -> (){
        //Send quit_ack
        //Disconnect
    }
}