use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use input_buffer::InputBuffer;
use bytes::Buf;

#[derive(Clone)]
pub struct PacketHandler {
    pub length: u32,
    pub on_receive: fn(&[u8]) -> ()
}

pub(crate) struct NetState{
    stream: TcpStream,
    addr: SocketAddr,
    buf: InputBuffer,
    handlers: HashMap<u8, PacketHandler>
}

impl NetState {
    pub(crate) fn new(stream: TcpStream, addr: SocketAddr) -> Self{
        NetState {stream, addr, buf: InputBuffer::new(), handlers: HashMap::new()}
    }

    pub(crate) fn register_handlers(&mut self, handlers: &HashMap<u8, PacketHandler>) -> (){
        for handler in handlers.iter(){
            self.handlers.insert(handler.0.clone(), handler.1.clone());
        }
    }

    pub fn receive_and_process(&mut self) {
        let bytes_read = match self.buf.read_from(&mut self.stream){
            Ok(bytes_read) => bytes_read,
            _ => {
                self.disconnect();
                return;
            }
        };

        if bytes_read > 0 {
            loop {
                let reader = self.buf.as_cursor_mut();
                //We have to try as this can be second loop iteration
                let packet_id = match reader.try_get_u8() {
                    Ok(x) => x,
                    _ => break // Need more data
                };

                let handler = match self.handlers.get(&packet_id) {
                    Some(handler) => handler,
                    _ => {
                        println!("Received unknown packet id {}", packet_id);
                        self.disconnect();
                        break;
                    }
                };
                let mut length = handler.length as usize;
                if length == 0 {
                    length = match reader.try_get_u32_le() {
                        Ok(x) => (x - 5) as usize, //Without id and length
                        _ => break // Need more data
                    };
                } else {
                    length -= 1; //Without id
                }

                if reader.remaining() < length {
                    break; // Need more data
                }
                let packet_data = &reader.chunk()[..length];
                (handler.on_receive)(packet_data);
                self.buf.advance(length)
            }
        }
    }

    pub fn disconnect(&mut self) {
        //TODO
    }
}