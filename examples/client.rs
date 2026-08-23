use znet::{IdentityConfig, NetworkConfig, ZNet};

fn main() -> std::io::Result<()> {
    let network = ZNet::new(
        NetworkConfig::new(9001),
        IdentityConfig::new("./data/client.key"),
    )?;

    network.start()?;

    loop {
        std::thread::park();
    }
}
