use crate::transport::{Connection, ConnectionId};
use std::collections::HashMap;
use std::io;
use std::net::{SocketAddr, TcpStream};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ConnectionManager {
    next_id: AtomicU64,
    connections: Mutex<HashMap<ConnectionId, Arc<Connection>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::new(1),
            connections: Mutex::new(HashMap::new()),
        }
    }

    pub fn connect(self: &Arc<Self>, address: SocketAddr) -> io::Result<ConnectionId> {
        let stream = TcpStream::connect(address)?;
        self.register(stream)
    }

    pub(crate) fn accept(self: &Arc<Self>, stream: TcpStream) -> io::Result<ConnectionId> {
        self.register(stream)
    }

    fn register(self: &Arc<Self>, stream: TcpStream) -> io::Result<ConnectionId> {
        let id = ConnectionId::new(self.next_id.fetch_add(1, Ordering::Relaxed));

        let connection = Arc::new(Connection::new(id, stream)?);

        self.connections
            .lock()
            .unwrap()
            .insert(id, Arc::clone(&connection));

        println!("Connection {} opened: {}", id.value(), connection.address());

        let manager = Arc::clone(self);

        thread::spawn(move || {
            if let Err(error) = connection.run() {
                println!("Connection {} error: {}", id.value(), error);
            }

            manager.remove(id);
        });

        Ok(id)
    }

    pub fn send(&self, id: ConnectionId, data: &[u8]) -> io::Result<()> {
        let connection = self
            .connections
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotConnected, "Connection not found"))?;

        connection.send(data)
    }

    pub fn connection_count(&self) -> usize {
        self.connections.lock().unwrap().len()
    }

    fn remove(&self, id: ConnectionId) {
        if let Some(connection) = self.connections.lock().unwrap().remove(&id) {
            println!("Connection {} closed: {}", id.value(), connection.address());
        }
    }
}
