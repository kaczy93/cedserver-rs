use crate::map::Map;
use std::net::TcpListener;
use crate::net_state::NetState;

pub struct CedServer {}

impl CedServer {
    pub fn run(mut map: Map) -> () {
        let listener = TcpListener::bind("0.0.0.0:2597").expect("Unable to bind to address");
        println!("Awaiting client connection...");
        //TODO: Support more than one client
        let (stream, addr) = listener.accept().expect("Unable to accept connection");
        println!("Client connected!");

        let mut netstate = NetState::new(stream, addr);
        loop {
            netstate.receive();
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
}