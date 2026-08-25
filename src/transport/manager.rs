use crate::transport::{Connection, ConnectionId};
use crate::wire::Frame;
use std::collections::HashMap;
use std::io;
use std::net::{SocketAddr, TcpStream};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ConnectionManager {
    next_id: AtomicU64,
    connections: Mutex<HashMap<ConnectionId, Arc<Connection>>>,
    frame_sender: Sender<(ConnectionId, Frame)>,
}

impl ConnectionManager {
    pub fn new(frame_sender: Sender<(ConnectionId, Frame)>) -> Self {
        Self {
            next_id: AtomicU64::new(1),
            connections: Mutex::new(HashMap::new()),
            frame_sender,
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
        let connection = Arc::new(Connection::new(stream)?);

        self.connections
            .lock()
            .unwrap()
            .insert(id, Arc::clone(&connection));

        println!("Connection {} opened: {}", id.value(), connection.address());

        let frame_sender = self.frame_sender.clone();
        let manager = Arc::clone(self);

        thread::spawn(move || {
            if let Err(error) = connection.run(id, frame_sender) {
                println!("Connection {} error: {}", id.value(), error);
            }

            manager.remove(id);
        });

        Ok(id)
    }

    pub fn send(&self, id: ConnectionId, frame: &Frame) -> io::Result<()> {
        let connection = self
            .connections
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotConnected, "Connection not found"))?;

        connection.send(frame)
    }

    pub fn connection_count(&self) -> usize {
        self.connections.lock().unwrap().len()
    }

    fn remove(&self, id: ConnectionId) {
        if let Some(connection) = self.connections.lock().unwrap().remove(&id) {
            println!("Connection {} closed: {}", id.value(), connection.address());
        }
    }

    pub(crate) fn address(&self, id: ConnectionId) -> Option<SocketAddr> {
        self.connections
            .lock()
            .unwrap()
            .get(&id)
            .map(|connection| connection.address())
    }
}
