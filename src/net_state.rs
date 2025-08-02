use std::collections::HashMap;
use std::io::Write;
use std::error::Error;
use std::net::{SocketAddr, TcpStream};
use input_buffer::InputBuffer;
use bytes::Buf;

#[derive(Clone)]
pub struct PacketHandler {
    pub length: usize,
    pub on_receive: fn(&NetState, &[u8]) -> ()
}

pub(crate) struct NetState{
    pub(crate) stream: TcpStream,
    addr: SocketAddr,
    recv_buffer: InputBuffer,
    handlers: HashMap<u8, PacketHandler>,
    running: bool,
    flush_pending: bool
}

impl NetState {
    pub(crate) fn new(stream: TcpStream, addr: SocketAddr) -> Self{
        NetState {stream, addr, recv_buffer: InputBuffer::new(), handlers: HashMap::new(), running: true, flush_pending: false}
    }

    pub(crate) fn register_handlers(&mut self, handlers: &HashMap<u8, PacketHandler>) -> (){
        for handler in handlers.iter(){
            self.handlers.insert(handler.0.clone(), handler.1.clone());
        }
    }

    pub fn receive(&mut self) {
        let bytes_read = match self.recv_buffer.read_from(&mut self.stream){
            Ok(bytes_read) => bytes_read,
            _ => {
                self.disconnect();
                return;
            }
        };

        if bytes_read > 0 {
            self.process_buffer()
        }
    }

    pub fn process_buffer(&mut self){
        loop {
            let mut data = self.recv_buffer.chunk();
            let packet_id = match data.try_get_u8() {
                Ok(x) => x,
                _ => break //No data
            };

            let handler = match self.handlers.get(&packet_id) {
                Some(handler) => handler,
                _ => {
                    println!("Received unknown packet id {}", packet_id);
                    self.disconnect();
                    break
                }
            };

            let (data_pos, packet_length): (usize, usize) = if handler.length != 0 {
                (1, handler.length)
            } else {
                match data.try_get_u32_le() {
                    Ok(x) => (5, x as usize),
                    _ => break // Need more data
                }
            };

            let data_length = packet_length - data_pos;
            if data.remaining() < data_length {
                break; // Need more data
            }

            (handler.on_receive)(self, &data[..data_length]);
            self.recv_buffer.advance(packet_length)
        }
    }

    pub fn send(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>>{
        let mut stream = &self.stream;
        stream.write(data)?;
        //TODO: Set flush pending
        Ok(())
    }

    pub fn disconnect(&mut self) {

        //TODO
    }
}