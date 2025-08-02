use crate::map::Map;
use std::collections::{HashMap};
use std::net::TcpListener;
use crate::net_state::{NetState, PacketHandler};

pub struct CedServer {}

impl CedServer {
    pub fn run(mut map: Map) -> () {
        let mut packet_handlers: HashMap<u8, PacketHandler> = HashMap::new();
        packet_handlers.insert(0x02, PacketHandler { length: 0, on_receive: NetState::on_connection_packet });
        let listener = TcpListener::bind("0.0.0.0:2597").expect("Unable to bind to address");
        println!("Awaiting client connection...");
        //TODO: Support more than one client
        let (stream, addr) = listener.accept().expect("Unable to accept connection");
        println!("Client connected!");

        let mut netstate = NetState::new(stream, addr);
        netstate.register_handlers(&packet_handlers);
        loop {
            netstate.receive();
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
}