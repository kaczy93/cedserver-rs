use crate::map::Map;
use std::collections::VecDeque;
use std::io::Read;
use std::net::TcpListener;

pub struct CedServer {
}

impl CedServer {
    pub fn run(mut map: Map) -> () {
        let listener = TcpListener::bind("0.0.0.0:2597").expect("Unable to bind to address");
        println!("Awaiting client connection...");
        //TODO: Support more than one client
        let (mut client_stream, _) = listener.accept().expect("Could not accept client");
        println!("Client connected!");
        std::thread::sleep(std::time::Duration::from_millis(1000));
        println!("Idling for data!");
        let mut data_buf: [u8; 2048] = [0; 2048];
        let mut data_queue: VecDeque<u8> = VecDeque::new();

        let bytes_read = client_stream.read(&mut data_buf).expect("Could not read from client");
        if bytes_read > 0 {
            println!("Read {} bytes", bytes_read);
            let data = &data_buf[..bytes_read];
            println!("{data:#?}");
            data_queue.extend(data);
            let packet_id = data_queue.front();
            let slice = &data_queue.as_slices().0[1..4];
            let packet_length = u32::from_le_bytes(slice.try_into().unwrap());
            let packet_data = data_queue.drain(..packet_length as usize);
            println!("Received data!");
            println!("{packet_data:#?}")
        }
    }
}