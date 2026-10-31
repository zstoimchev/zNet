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

            loop {
                match connection.receive() {
                    Ok(Some(data)) => {
                        let bytes_length = data.len();
                        let message = String::from_utf8_lossy(&data);
                        println!("Received {} bytes: {}", bytes_length, message);

                        if let Err(error) = connection.send(&data) {
                            eprintln!("Send error: {}", error);
                            break;
                        }
                    }

                    Ok(None) => {
                        println!("Client disconnected");
                        break;
                    }

                    Err(error) => {
                        eprintln!("Connection error: {}", error);
                        break;
                    }
                }
            }
        });
    }

    Ok(())
}
