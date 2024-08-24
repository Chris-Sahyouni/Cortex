use std::{net::{TcpListener}};


fn main() {
    let listener: TcpListener = TcpListener::bind("0.0.0.0:32503").unwrap();
    
    for stream in listener.incoming() {

    }
}