use crate::map::Map;
use crate::net_state::NetState;
use std::cell::RefCell;
use std::net::TcpListener;
use std::rc::Rc;

pub struct CedServer {}

impl CedServer {
    pub fn run(map: Map) -> () {
        //Should we pass map mut ref to every receive call to get rid of rc<refcell<>>?
        let map = Rc::new(RefCell::new(map));
        let listener = TcpListener::bind("0.0.0.0:2597").expect("Unable to bind to address");
        println!("Awaiting client connection...");
        //TODO: Support more than one client
        let (stream, addr) = listener.accept().expect("Unable to accept connection");
        println!("Client connected!");

        let mut netstate = NetState::new(stream, addr, map.clone());
        while netstate.is_running() {
            netstate.receive();
            netstate.flush().expect("Unable to flush socket"); //TODO: Better error handling
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        //Final receive and flush
        netstate.receive();
        netstate.flush().unwrap();
    }
}