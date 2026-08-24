use crate::config::NetworkConfig;
use crate::transport::{ConnectionId, ConnectionManager, Server};

use crate::identity::NodeIdentity;
use crate::peer::{PeerId, PeerRegistry};
use crate::wire::{Frame, FrameType};
use std::io;
use std::net::SocketAddr;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

pub struct NetworkManager {
    config: NetworkConfig,
    identity: NodeIdentity,
    peers: PeerRegistry,
    connections: Arc<ConnectionManager>,
    frame_receiver: Mutex<Option<Receiver<(ConnectionId, Frame)>>>,
}

impl NetworkManager {
    pub fn new(config: NetworkConfig, identity: NodeIdentity) -> Self {
        let (frame_sender, frame_receiver) = mpsc::channel();
        let connections = Arc::new(ConnectionManager::new(frame_sender));

        Self {
            config,
            identity,
            peers: PeerRegistry::new(),
            connections,
            frame_receiver: Mutex::new(Some(frame_receiver)),
        }
    }

    pub fn start(self: &Arc<Self>) -> io::Result<()> {
        self.start_frame_handler();
        self.start_server()?;
        self.connect_bootstrap_peers();
        Ok(())
    }

    fn start_frame_handler(self: &Arc<Self>) {
        let receiver = self
            .frame_receiver
            .lock()
            .unwrap()
            .take()
            .expect("NetworkManager already started");

        let manager = Arc::clone(self);

        thread::spawn(move || {
            while let Ok((connection_id, frame)) = receiver.recv() {
                manager.handle_frame(connection_id, frame);
            }
        });
    }

    fn handle_frame(&self, connection_id: ConnectionId, frame: Frame) {
        println!(
            "NetworkManager received {:?} from connection {}: {:?}",
            frame.frame_type(),
            connection_id.value(),
            String::from_utf8_lossy(frame.payload())
        );
    }

    fn start_server(self: &Arc<Self>) -> io::Result<()> {
        let server = Server::bind(self.config.listen_address(), Arc::clone(&self.connections))?;

        thread::spawn(move || {
            server.run();
        });

        Ok(())
    }

    fn connect_bootstrap_peers(self: &Arc<Self>) {
        for address in self.config.bootstrap_peers() {
            if let Err(error) = self.connect(*address) {
                println!("Failed to connect to bootstrap {}: {}", address, error);
            }
        }
    }

    pub fn connect(self: &Arc<Self>, address: SocketAddr) -> io::Result<ConnectionId> {
        self.connections.connect(address)
    }

    pub fn send(&self, connection: ConnectionId, data: &[u8]) -> io::Result<()> {
        let frame = Frame::new(FrameType::Data, data.to_vec());
        self.connections.send(connection, &frame)
    }

    pub(crate) fn local_peer_id(&self) -> PeerId {
        self.identity.peer_id()
    }
}
