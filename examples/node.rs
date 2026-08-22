use std::thread;
use znet::znet::ZNet;

fn main() {
    ZNet::new("127.0.0.1:9000".to_string()).start();

    loop {
        thread::park();
    }
}
