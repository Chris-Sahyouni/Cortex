use std::{net::{TcpListener, TcpStream}, io::{prelude::*, BufReader}};
use cortex::{CortexArgs, CortexCommands, JobState};

const SERVER_IP_ADDR: &str = "127.0.0.1:4444";

fn main() {
    // use a threadpool to handle the connections
    let listener = TcpListener::bind(SERVER_IP_ADDR).unwrap();
    for stream in listener.incoming() {
        // connection attempt may not be successful so we pattern match it to ensure its Ok
        if let Ok(_stream) = stream {
            handle_connection(_stream);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut serialized_args = String::new();
    buf_reader.read_line(&mut serialized_args).unwrap();
    let args: CortexArgs = serde_json::from_str(serialized_args.as_str()).unwrap();
}

/* ---------------------------------- Tests --------------------------------- */

#[cfg(test)]
mod tests {

}