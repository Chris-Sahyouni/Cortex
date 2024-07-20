use std::{net::{TcpListener, TcpStream}, io::{prelude::*, BufReader}};
use cortex::{CortexArgs, CortexCommands, JobState};

const SERVER_IP_ADDR: &str = "127.0.0.1:4444";

fn main() {
    let listener = TcpListener::bind(SERVER_IP_ADDR).unwrap();
    for stream in listener.incoming() {
        // connection attempt may not be successful so we pattern match it to ensure its Ok
        if let Ok(_stream) = stream {
            handle_connection(_stream);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    ()
}