use bytes::Buf;
use input_buffer::InputBuffer;
use std::cell::Cell;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::net::{SocketAddr, TcpStream};

type PacketHandler = fn(&NetState, &[u8]) -> Result<(), Box<dyn Error>>;

pub(crate) struct NetState{
    pub(crate) stream: TcpStream,
    addr: SocketAddr,
    pub(crate) recv_buffer: InputBuffer,
    running: Cell<bool>,
    flush_pending: Cell<bool>
}

impl NetState {

    pub(crate) fn new(stream: TcpStream, addr: SocketAddr) -> Self{
        NetState {stream, addr, recv_buffer:InputBuffer::new(), running: Cell::new(true), flush_pending: Cell::new(false)}
    }

    pub(crate) fn receive(&mut self) -> bool {
        let bytes_read = self.recv_buffer.read_from(&mut self.stream).unwrap_or_else(|err|{
            self.disconnect(err);
            0
        });
        if bytes_read > 0 {
            if let Err(err) = self.process_buffer() {
                self.disconnect(err)
            }
        }
        //Is this a good return value here?
        self.running.get() 
    }

    fn process_buffer(&mut self) -> Result<(), Box<dyn Error>>{
        loop {
            let mut data = self.recv_buffer.chunk(); //TODO: Can we use cursor so we don't have to track pos ourself?
            let packet_id = match data.try_get_u8() {
                Ok(x) => x,
                _ => break //No data
            };

            let (packet_length, packet_handler) = self.get_packet_handler(packet_id)?;

            let (data_pos ,packet_length): (usize, usize) = if packet_length != 0 {
                (1, packet_length)
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

            packet_handler(self, &data.chunk()[..data_length])?;
            self.recv_buffer.advance(packet_length);
        }
        Ok(())
    }

    pub fn get_packet_handler(&self, packet_id: u8) -> Result<(usize, PacketHandler), NetStateError> {
        match packet_id{
            0x02 => Ok((0, NetState::on_connection_packet)),
            _ => Err(NetStateError(format!("Unknown packet {packet_id}")))
        }
    }

    pub fn send(&self, data: &[u8]) -> Result<(), Box<dyn Error>>{
        let mut stream = &self.stream;
        let bytes_written = stream.write(data)?;
        if bytes_written != data.len() {
            return Err(NetStateError(String::from("Failed to write all packet data")).into())
        }
        self.flush_pending.set(true);
        Ok(())
    }

    pub fn disconnect(&self, reason: impl Display) {
        println!("{}: Disconnecting. Cause: {}", &self.addr, reason);
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