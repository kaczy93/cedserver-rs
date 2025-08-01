use std::io::Write;
use crate::{AccessLevel, LoginState};
use crate::net_state::NetState;
use bytes::{BufMut};

impl NetState{
    pub fn send_login_response(&mut self, state: LoginState){
        let mut data: &mut[u8] = &mut [0; 12];
        data.put_u8(0x02);                  //PacketID
        data.put_u32(14);                   //Length
        data.put_u8(0x03);                  //Command(LoginResponse)
        //TODO
        data.put_u8(AccessLevel::Normal as u8);//AccessLevel
        data.put_u16(896);                  //Map Width
        data.put_u16(512);                  //Map Height
        //Account restrictions TODO
        data.put_u8(0); //No restrictions
        self.stream.write(data).expect("Failed to send login response");
    }
}