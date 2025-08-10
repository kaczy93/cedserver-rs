use crate::buf_utf8::BufUtf8;
use crate::enums::{AccessLevel, LoginState};
use crate::net_state::NetState;
use bytes::{Buf, BufMut};
use std::error::Error;
use crate::server::CedServer;

impl NetState {
    pub(crate) fn on_connection_packet(&self, cedserver: &mut CedServer, mut data: &[u8]) -> Result<(), Box<dyn Error>> {
        let cmd_id = data.get_u8();
        match cmd_id {
            0x03 => self.on_login_request(cedserver, data),
            0x05 => self.on_quit(cedserver, data),
            _ => panic!("Unknown connection handler command {cmd_id}")
        }
    }

    fn on_login_request(&self, cedserver: &mut CedServer, mut data: &[u8]) -> Result<(), Box<dyn Error>> {
        println!("on_login_request");
        let username = data.get_str_nul();
        println!("User: {username}");
        let password = data.get_str_nul();
        println!("Password: {password}");
        //TODO: Check if valid
        self.send_login_response(cedserver, LoginState::Ok)
    }

    pub fn send_login_response(&self, cedserver: &mut CedServer, state: LoginState) -> Result<(), Box<dyn Error>>{
        let mut data = [0u8; 14];
        let mut writer = &mut data[..];
        writer.put_u8(0x02);                  //PacketID
        writer.put_u32_le(14);                   //Length
        writer.put_u8(0x03);                  //Command(LoginResponse)
        writer.put_u8(state as u8);
        writer.put_u8(AccessLevel::Normal as u8); //TODO: Real access level
        writer.put_u16_le(cedserver.map.width());
        writer.put_u16_le(cedserver.map.height());
        //Account restrictions TODO
        writer.put_u16_le(0); //No restrictions
        self.send(&data)?;
        Ok(())
    }

    fn on_quit(&self, cedserver: &mut CedServer, _data: &[u8]) -> Result<(), Box<dyn Error>>{
        println!("on_quit_packet");
        self.send_quit_ack()?;
        self.disconnect("Requested By Client");
        Ok(())
    }

    fn send_quit_ack(&self) -> Result<(), Box<dyn Error>> {
        let mut data = [0u8; 6];
        let mut writer = &mut data[..];
        writer.put_u8(0x02);
        writer.put_u32_le(6);
        writer.put_u8(0x05);
        self.send(&data)?;
        Ok(())
    }
}