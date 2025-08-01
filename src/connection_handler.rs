use bytes::Buf;
use crate::bytes_utf8::BytesUtf8;

pub(crate) fn on_connection_packet(mut data: &[u8]) -> (){
    let cmd_id = data.get_u8();
    match cmd_id {
        0x03 => on_login_request(data),
        0x05 => on_quit(data),
        _ => panic!("Unknown connection handler command {cmd_id}")
    }
}

fn on_login_request(mut data: &[u8]) -> (){
    let username = data.get_str_nul();
    println!("User: {username}");
    let password = data.get_str_nul();
    println!("Password: {password}");
    //TODO: 
}

fn on_quit(mut data: &[u8]) -> (){
    //Send quit_ack
    //Disconnect
}