use std::error::Error;
use crate::{AccessLevel, LoginState};
use crate::net_state::NetState;
use bytes::{BufMut};

impl NetState{
    pub fn send_login_response(&self, state: LoginState) -> Result<(), Box<dyn Error>>{
        let mut data: &mut[u8] = &mut [0; 14];
        data.put_u8(0x02);                  //PacketID
        data.put_u32(14);                   //Length
        data.put_u8(0x03);                  //Command(LoginResponse)
        data.put_u8(state as u8);              //State
        //TODO
        data.put_u8(AccessLevel::Normal as u8);//AccessLevel
        data.put_u16(896);                  //Map Width
        data.put_u16(512);                  //Map Height
        //Account restrictions TODO
        data.put_u8(0); //No restrictions
        self.send(data)?;
        Ok(())
    }
}