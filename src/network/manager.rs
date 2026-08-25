use crate::config::NetworkConfig;
use crate::transport::{ConnectionId, ConnectionManager, Server};

use crate::Peer;
use crate::identity::NodeIdentity;
use crate::peer::{PeerId, PeerRegistry};
use crate::protocol::{Handshake, HandshakeKind, PROTOCOL_VERSION};
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
        let result = match frame.frame_type() {
            FrameType::Handshake => self.handle_handshake(connection_id, frame.payload()),
            FrameType::Data => self.handle_data(connection_id, frame.payload()),
        };

        if let Err(error) = result {
            println!(
                "Connection {} protocol error: {}",
                connection_id.value(),
                error
            );
        }
    }

    fn handle_handshake(&self, connection_id: ConnectionId, payload: &[u8]) -> io::Result<()> {
        let handshake = Handshake::decode(payload)?;
        if handshake.version() != PROTOCOL_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unsupported protocol version",
            ));
        }

        let remote_address = self
            .connections
            .address(connection_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotConnected, "Connection not found"))?;

        let peer_id = PeerId::from_public_key(handshake.public_key());
        if peer_id == self.local_peer_id() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Cannot connect to self",
            ));
        }

        let peer_address = SocketAddr::new(remote_address.ip(), handshake.listen_port());
        let peer = Peer::new(peer_id, peer_address);

        self.peers.register(peer, connection_id);

        println!("Peer registered: {:?} at {}", peer_id, peer_address);

        if handshake.kind() == HandshakeKind::Hello {
            self.send_handshake(connection_id, HandshakeKind::Ack)?;
        }

        Ok(())
    }

    fn send_handshake(&self, connection_id: ConnectionId, kind: HandshakeKind) -> io::Result<()> {
        let handshake = Handshake::new(kind, self.config.port(), self.identity.public_key_bytes());
        let frame = Frame::new(FrameType::Handshake, handshake.encode());
        self.connections.send(connection_id, &frame)
    }

    fn handle_data(&self, connection_id: ConnectionId, payload: &[u8]) -> io::Result<()> {
        let peer = self
            .peers
            .peer_by_connection(connection_id)
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::PermissionDenied, "Handshake not completed")
            })?;

        println!(
            "Received data from {:?}: {:?}",
            peer.id(),
            String::from_utf8_lossy(payload)
        );
        
        Ok(())
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
        let connection_id = self.connections.connect(address)?;
        self.send_handshake(connection_id, HandshakeKind::Hello)?;
        Ok(connection_id)
    }

    pub fn send(&self, connection: ConnectionId, data: &[u8]) -> io::Result<()> {
        let frame = Frame::new(FrameType::Data, data.to_vec());
        self.connections.send(connection, &frame)
    }

    pub(crate) fn local_peer_id(&self) -> PeerId {
        self.identity.peer_id()
    }
}
