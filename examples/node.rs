use std::thread;
use znet::znet::ZNet;

fn main() {
    let network = ZNet::new("127.0.0.1:9000".to_string());

    network.start();

    loop {
        thread::park();
    }
}
