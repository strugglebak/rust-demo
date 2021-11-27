use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // we declare a buffer on the stack to hold the data that is read in
    // We’ve made the buffer 1024 bytes in size
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // takes a &[u8] and produces a String from it
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
