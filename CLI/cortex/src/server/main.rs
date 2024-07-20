use std::net::TcpListener;

const SERVER_IP_ADDR: &str = "127.0.0.1:4444";

fn main() {
    let listener = TcpListener::bind(SERVER_IP_ADDR).unwrap();
    
}