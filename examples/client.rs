use std::net::SocketAddr;

use znet::{IdentityConfig, NetworkConfig, ZNet};

fn main() -> std::io::Result<()> {
    let network = ZNet::new(
        NetworkConfig::new(9001),
        IdentityConfig::new("./data/client.key"),
    )?;

    network.start()?;

    let server_address: SocketAddr = "127.0.0.1:9000".parse().unwrap();
    let connection = network.connect(server_address)?;
    network.send(connection, b"Hello from client")?;

    loop {
        std::thread::park();
    }
}
