use std::io::Write;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9000").expect("Failed to connect to server");

    println!("Connected to {}", stream.peer_addr().unwrap());

    stream
        .write_all(b"Hello from client")
        .expect("Failed to send message");
}
