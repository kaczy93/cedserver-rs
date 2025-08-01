use crate::map::Map;
use std::collections::{HashMap};
use std::net::TcpListener;
use input_buffer::InputBuffer;
use bytes::Buf;
use crate::bytes_utf8::BytesUtf8;

struct PacketHandler {
    length: u32,
    on_receive: fn(&[u8]) -> ()
}

pub struct CedServer {
}

impl CedServer {
    pub fn run(mut map: Map) -> () {
        let mut packet_handlers: HashMap<u8, PacketHandler> = HashMap::new();
        packet_handlers.insert(0x02, PacketHandler { length: 0, on_receive: Self::on_connection_packet });
        let listener = TcpListener::bind("0.0.0.0:2597").expect("Unable to bind to address");
        println!("Awaiting client connection...");
        //TODO: Support more than one client
        let (mut client_stream, _) = listener.accept().expect("Could not accept client");
        println!("Client connected!");

        let mut data_buf = InputBuffer::new();
        loop{
            let bytes_read = data_buf.read_from(&mut client_stream).expect("Could not read from client");
            if bytes_read > 0 {
                println!("Read {} bytes", bytes_read);
                loop {
                    // Cursor to seek data
                    let reader = data_buf.as_cursor_mut();
                    let packet_id = match reader.try_get_u8() {
                        Ok(x) => x,
                        _ => break
                    };

                    let handler = packet_handlers.get(&packet_id).expect(&format!("Invalid packet id {packet_id}"));
                    let mut length = handler.length as usize;
                    if length == 0 {
                        length = match reader.try_get_u32_le(){
                            Ok(x) => (x - 5) as usize, //Without id and length
                            _ => break
                        };
                        if reader.remaining() < 5 {
                            break; //wait for more data
                        }
                    }
                    else {
                        length -= 1; //Without id
                    }

                    if reader.remaining() < length {
                        break; //wait for more data
                    }
                    let packet_data = &reader.chunk()[..length];
                    (handler.on_receive)(packet_data);
                    data_buf.advance(length)
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    fn on_connection_packet(mut data: &[u8]) -> (){
        let cmdId = data.get_u8();
        match cmdId {
            0x03 => {
                let username = data.get_str_nul();
                println!("User: {username}");
                let password = data.get_str_nul();
                println!("Password: {password}");
            }
            0x05 => {
                //Send quit_ack
                //Disconnect
            }
            _ => panic!("Unknown connection handler command {cmdId}")
        }
    }
}