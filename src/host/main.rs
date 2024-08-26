use std::{net::{TcpListener}, BufReader};


fn main() {
    let listener: TcpListener = TcpListener::bind("0.0.0.0:32503").unwrap();

    // don't forget there must be a way to authenticate the server

    for stream in listener.incoming() {
        let buf_reader = BufReader::new(&mut stream);
        let stream_contents = String::new();
        if let Ok(()) = buf_reader.read_line(&mut stream_contents) {
            if stream_contents.starts_with("Commit-Request:") {
                let job_id = stream_contents.split(":").collect()[1];
                let commit_response = "Committed:" + job_id;
                stream.write_all(commit_response.as_bytes());
            }
        }
    }
}