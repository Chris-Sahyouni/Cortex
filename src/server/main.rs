use std::{io::{prelude::*}, sync::mpsc::channel, error::Error};
use cortex::{CortexArgs, CortexCommands, JobState};
use tokio::{io::{AsyncBufReadExt, BufReader}, net::{TcpListener, TcpStream}};

extern crate mongodb;
use mongodb::{bson::doc, options::ClientOptions, Client};

const SERVER_ADDR: &str = "127.0.0.1:32503";
const DB_CONN_STRING: &str = "mongodb://localhost:27017";


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(SERVER_ADDR).await.unwrap();
    // let (sender, receiver) = channel();

    let db_conn = ClientOptions::parse(DB_CONN_STRING).await.unwrap();

    loop {
        // connection attempt may not be successful so we pattern match it to ensure its Ok
        if let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(async move {
                // use the result here somehow
                handle_connection(stream);
            });
        }
    }
}

async fn db_connection() -> Result<Client, Box<dyn Error>> {
    let client_options = ClientOptions::parse(DB_CONN_STRING).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}



async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {


    let mut buf_reader = BufReader::new(&mut stream);
    let mut serialized_args = String::new();
    buf_reader.read_line(&mut serialized_args).await?;
    let args: CortexArgs = serde_json::from_str(serialized_args.as_str()).unwrap();



    Ok(())
}

/* ---------------------------------- Tests --------------------------------- */

#[cfg(test)]
mod tests {

}