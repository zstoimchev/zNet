use znet::{NetworkConfig, ZNet};

fn main() -> std::io::Result<()> {
    let network = ZNet::new(NetworkConfig::new(9001));

    network.start()?;

    let connection = network.connect("127.0.0.1:9000".parse().unwrap())?;

    network.send(connection, b"Hello from zNet")?;

    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(())
}
