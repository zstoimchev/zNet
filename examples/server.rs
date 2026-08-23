use znet::{NetworkConfig, ZNet};

fn main() -> std::io::Result<()> {
    let network = ZNet::new(NetworkConfig::new(9000));

    network.start()?;

    loop {
        std::thread::park();
    }
}
