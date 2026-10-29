use crate::transport::connection::Connection;
use std::io::Result;
use std::net::TcpListener;
use std::thread;

pub fn start() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8088")?;

    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || {
            let mut connection = Connection::new(stream);

            if let Err(error) = connection.receive() {
                eprintln!("Connection error: {error}");
            }
        });
    }

    Ok(())
}
