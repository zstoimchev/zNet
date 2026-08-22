use znet::server::Server;

fn main() {
    Server::new("127.0.0.1:9000".to_string()).start();
}
