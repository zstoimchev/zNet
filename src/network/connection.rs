use crate::network::manager::NetworkManager;
use crate::network::peer::Peer;
use crate::protocol::frame::Frame;
use crate::protocol::handshake::HandshakeProtocol;
use crate::protocol::message::MessageType;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct PeerConnection {
    address: SocketAddr,
    read_stream: Mutex<TcpStream>,
    write_stream: Mutex<TcpStream>,
}

impl PeerConnection {
    pub fn new(stream: TcpStream) -> std::io::Result<Self> {
        let address = stream.peer_addr()?;
        let write_stream = stream.try_clone()?;

        Ok(Self {
            address,
            read_stream: Mutex::new(stream),
            write_stream: Mutex::new(write_stream),
        })
    }

    pub fn start(self: &Arc<Self>, network_manager: Arc<NetworkManager>) {
        let connection = Arc::clone(self);

        thread::spawn(move || {
            connection.run(network_manager);
        });
    }

    fn run(self: Arc<Self>, network_manager: Arc<NetworkManager>) {
        println!("Connection opened: {}", self.address);

        let peer = match self.perform_handshake(&network_manager) {
            Ok(peer) => peer,
            Err(error) => return self.handle_handshake_error(error),
        };

        let public_key = peer.public_key().to_vec();
        println!("Connected to peer with public key: {:02x?}", public_key);
        network_manager.register_peer(peer, Arc::clone(&self));
        self.listen(public_key, &network_manager);
    }

    fn perform_handshake(&self, network_manager: &NetworkManager) -> std::io::Result<Peer> {
        let mut stream = self.read_stream.lock().unwrap();
        HandshakeProtocol::perform(
            &mut *stream,
            network_manager.local_port(),
            network_manager.local_public_key(),
        )
    }

    fn listen(&self, public_key: Vec<u8>, network_manager: &NetworkManager) {
        let mut stream = self.read_stream.lock().unwrap();

        loop {
            let frame = match Frame::read(&mut *stream) {
                Ok(frame) => frame,

                Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => {
                    return self.handle_close(&public_key, network_manager);
                }

                Err(error) => {
                    return self.handle_error(&public_key, network_manager, error);
                }
            };

            self.handle_frame(frame);
        }
    }

    fn handle_frame(&self, frame: Frame) {
        let (message_type, flags, payload) = frame.into_parts();

        match message_type {
            MessageType::Handshake => {
                println!("Unexpected handshake from {}", self.address);
            }

            MessageType::PeerDiscoveryRequest => {
                println!("Peer discovery request");
            }

            MessageType::PeerDiscoveryResponse => {
                println!("Peer discovery response");
            }

            MessageType::Heartbeat => {
                println!("Heartbeat");
            }

            MessageType::Data => {
                println!("{}: {}", self.address, String::from_utf8_lossy(&payload),);
            }
        }
    }

    pub fn send(&self, message_type: MessageType, payload: Vec<u8>) -> std::io::Result<()> {
        let frame = Frame::new(message_type, 0, payload);

        let mut stream = self.write_stream.lock().unwrap();

        frame.write(&mut *stream)
    }

    fn handle_close(&self, public_key: &[u8], network_manager: &NetworkManager) {
        network_manager.remove_connection(public_key);
        println!("Connection closed: {}", self.address);
    }

    fn handle_error(
        &self,
        public_key: &[u8],
        network_manager: &NetworkManager,
        error: std::io::Error,
    ) {
        println!("Connection error {}: {}", self.address, error);
        self.handle_close(public_key, network_manager);
    }

    fn handle_handshake_error(&self, error: std::io::Error) {
        println!("Handshake failed {}: {}", self.address, error);
    }
}
