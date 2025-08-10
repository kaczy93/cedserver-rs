use crate::enums::AccessLevel;
use crate::server::CedServer;
use bytes::Buf;
use input_buffer::InputBuffer;
use std::cell::Cell;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::net::{SocketAddr, TcpStream};

pub(crate) struct NetState{
    stream: TcpStream,
    addr: SocketAddr,
    recv_buffer: InputBuffer,
    running: Cell<bool>,
    flush_pending: Cell<bool>
}

impl NetState{
    pub(crate) fn new(stream: TcpStream, addr: SocketAddr) -> Self{
        NetState {stream, addr, recv_buffer:InputBuffer::new(), running: Cell::new(true), flush_pending: Cell::new(false)}
    }

    pub(crate) fn receive(&mut self, cedserver: &mut CedServer) -> bool {
        let bytes_read = self.recv_buffer.read_from(&mut self.stream).unwrap_or_else(|err|{
            self.disconnect(err);
            0
        });
        if bytes_read > 0 {
            if let Err(err) = self.process_buffer(cedserver) {
                self.disconnect(err)
            }
        }
        //Is this a good return value here?
        self.running.get()
    }

    pub fn is_running(&self) -> bool {
        self.running.get()
    }

    fn process_buffer(&mut self, cedserver: &mut CedServer) -> Result<(), Box<dyn Error>>{
        loop {
            let mut data = self.recv_buffer.chunk(); //TODO: Can we use cursor so we don't have to track pos ourself?
            let packet_id = match data.try_get_u8() {
                Ok(x) => x,
                _ => break //No data
            };

            let mut packet_length = self.get_packet_length(packet_id)?;
            let mut data_pos = 1;

            if packet_length == 0 {
                if data.remaining() < 4 {
                    break; // Need more data
                }
                packet_length = data.get_u32_le() as usize;
                data_pos = 5;
            }

            let data_length = packet_length - data_pos;
            if data.remaining() < data_length {
                break; // Need more data
            }

            self.handle_packet(cedserver, packet_id, &data.chunk()[..data_length])?;
            self.recv_buffer.advance(packet_length);
        }
        Ok(())
    }

    fn get_packet_length(&self, packet_id: u8) -> Result<usize, Box<dyn Error>>{
        match packet_id{
            0x02 => Ok(0),
            0x04 => Ok(0),
            _ => Err(NetStateError("Unknown packet".to_string()).into())
        }
    }

    fn handle_packet(&self, cedserver: &mut CedServer, packet_id: u8, data: &[u8]) -> Result<(), Box<dyn Error>> {
        match packet_id{
            0x02 => self.on_connection_packet(cedserver, data),
            0x04 => self.on_blocks_request_packet(cedserver, data),
            _ => Err(NetStateError("Unknown packet".to_string()).into())
        }
    }

    pub(crate) fn send(&self, data: &[u8]) -> Result<(), Box<dyn Error>>{
        //Do we need input buffer as send_buffer?
        let mut stream = &self.stream;
        let bytes_written = stream.write(data)?;
        if bytes_written != data.len() {
            return Err(NetStateError(String::from("Failed to write all packet data")).into())
        }
        self.flush_pending.set(true);
        Ok(())
    }

    pub(crate) fn flush(&self) -> Result<(), std::io::Error>{
        if self.flush_pending.get() {
            let mut stream = &self.stream;
            stream.flush()?;
        }
        Ok(())
    }

    pub(crate) fn assert_access(&self, _access_level: AccessLevel) -> Result<(), NetStateError> {
        //TODO: Check current netstate access level
        Ok(())
    }
    
    pub fn disconnect(&self, reason: impl Display) {
        println!("{}: Disconnected. Cause: {}", &self.addr, reason);
        self.running.set(false);
    }
}

#[derive(Debug)]
pub(crate) struct NetStateError(pub String);

impl Display for NetStateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NetState Error: {}", self.0)
    }
}

impl Error for NetStateError {}