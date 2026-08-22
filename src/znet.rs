use crate::server::Server;
use std::thread;

pub struct ZNet {
    server: Server,
}

impl ZNet {
    pub fn new(address: String) -> Self {
        Self {
            server: Server::new(address),
        }
    }

    pub fn start(&self) {
        let server = self.server.clone();

        thread::spawn(move || {
            server.start();
        });
    }
}
