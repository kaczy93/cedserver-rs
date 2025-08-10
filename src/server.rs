use crate::map::Map;
use crate::net_state::NetState;
use std::net::TcpListener;

pub struct CedServer {
    pub map: Map
}

impl CedServer {
    pub fn new(map: Map) -> Self{
        CedServer{map}
    }
    pub fn run(&mut self) -> () {
        let listener = TcpListener::bind("0.0.0.0:2597").expect("Unable to bind to address");
        println!("Awaiting client connection...");
        //TODO: Support more than one client
        let (stream, addr) = listener.accept().expect("Unable to accept connection");
        println!("Client connected!");

        let mut netstate = NetState::new(stream, addr);
        while netstate.is_running() {
            netstate.receive(self);
            netstate.flush().expect("Unable to flush socket"); //TODO: Better error handling
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        //Final receive and flush
        netstate.receive(self);
        netstate.flush().unwrap();
    }
}