use std::{io::{prelude::*}, sync::mpsc::channel, error::Error};
use clap::builder::styling::Color;
use cortex::{CortexArgs, CortexCommands, JobState };
use tokio::{io::{AsyncBufReadExt, BufReader}, net::{TcpListener, TcpStream}};

mod host;
use host::Host;

extern crate mongodb;
use mongodb::{bson::{doc, Document}, options::ClientOptions, Client, Collection, Cursor};

const SERVER_ADDR: &str = "127.0.0.1:32503";
const DB_CONN_STRING: &str = "mongodb://localhost:27017";


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(SERVER_ADDR).await.unwrap();
    // let (sender, receiver) = channel();

    let db_conn = db_connection().await.unwrap();

    loop {
        // uses Arc internally, so its fine to clone this in a loop
        let db_conn = db_conn.clone();

        // connection attempt may not be successful so we pattern match it to ensure its Ok
        if let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(async move {
                if let Ok(()) = handle_connection(stream, db_conn).await {
                    println!("task succeeded");
                }
            });
        }
    }
}

async fn db_connection() -> Result<Client, Box<dyn Error>> {
    let mut client_options = ClientOptions::parse(DB_CONN_STRING).await?;
    client_options.app_name = Some(String::from("Cortex"));
    let client = Client::with_options(client_options)?;
    Ok(client)
}



async fn handle_connection(mut stream: TcpStream, db_conn: Client) -> Result<(), Box<dyn Error>> {

    // serialize the bytes from the stream into a CortexArgs
    let mut buf_reader = BufReader::new(&mut stream);
    let mut serialized_args = String::new();
    buf_reader.read_line(&mut serialized_args).await?;
    let args: CortexArgs = serde_json::from_str(serialized_args.as_str()).unwrap();

    // query the db
    let matching_hosts = queryHosts(args, db_conn).await?;

    
    Ok(())
}

async fn queryHosts(args: CortexArgs, db_conn: Client) -> Result<Cursor<Host>, Box<dyn Error>> {
    let hosts: Collection<Host> = db_conn.database("hosts_db").collection("hosts");
    let filter: Document;
    if args.make == "ANY" {
        filter = doc! {
            "gpus": doc! { "$gte": args.gpus},
            "available": true
        };
    } else {
        filter = doc! {
        "gpus": doc! { "$gte": args.gpus},
        "make": args.make.to_uppercase(),
        "available": true
    };
    }
    // for now we return double the redundancy level documents to account for hosts that do not respond
    let matching_hosts = hosts.find(filter).batch_size(args.redundancy * 2).await?;
    Ok(matching_hosts)
}

/* ---------------------------------- Tests --------------------------------- */

#[cfg(test)]
mod tests {

}