use znet::{IdentityConfig, NetworkConfig, ZNet};

fn main() -> std::io::Result<()> {
    let network = ZNet::new(
        NetworkConfig::new(9000),
        IdentityConfig::new("./data/server.key"),
    )?;

    network.start()?;

    loop {
        std::thread::park();
    }
}
