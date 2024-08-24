use core::borrow;
use std::{error::Error, io::prelude::*, ops::Deref, sync::{mpsc::channel, Arc}};
use clap::builder::styling::Color;
use cortex::{CortexArgs, CortexCommands, JobState};
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::{TcpListener, TcpStream}, sync::{Mutex, MutexGuard}};
use futures::stream::StreamExt;

mod host;
use host::{Host, HostState};

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
        // uses Arc internally, so its fine to clone db_conn in a loop
        let db_conn = db_conn.clone();

        // connection attempt may not be successful so we pattern match it to ensure its Ok
        if let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(async move {
                if let Ok(()) = handle_connection(stream, &db_conn).await {
                    println!("task succeeded");
                }
            });
        }
    }
}


async fn handle_connection(mut stream: TcpStream, db_conn: &Client) -> Result<(), Box<dyn Error>> {

    // serialize the bytes from the stream into a CortexArgs
    let mut buf_reader = BufReader::new(&mut stream);
    let mut serialized_args = String::new();
    buf_reader.read_line(&mut serialized_args).await?;
    let args: CortexArgs = serde_json::from_str(serialized_args.as_str()).unwrap();

    // query the db
    let mut hosts_cursor = query_hosts(args.clone(), &db_conn).await?;

    if let Ok(()) = two_phase_commit(&mut hosts_cursor, args.redundancy.clone(), args.id.clone(), db_conn).await {

    }

    Ok(())
}

async fn two_phase_commit(cursor: &mut Cursor<Host>, redundancy: u32, job_id: String, db_conn: &Client) -> Result<(), Box<dyn Error>> {

    let committed_hosts: Arc<Mutex<Vec<Host>>> = Arc::new(Mutex::new(Vec::new()));


    while let Some(Ok(host)) = cursor.next().await {


        // easier to just make these in every iteration
        let commit_req = String::from("Commit-Request:") + job_id.as_str();
        let committed_res = String::from("Committed:") + job_id.as_str();

        let committed_hosts = Arc::clone(&committed_hosts);

        tokio::spawn(async move {
            let host_socket = host.ip.to_string() + ":32503";
            if let Ok(mut stream) = TcpStream::connect(host_socket).await {
                if let Ok(()) = stream.write_all(commit_req.as_bytes()).await {
                    let mut buf_reader = BufReader::new(&mut stream);
                    let mut response = String::new();
                    if let Ok(_bytes_read) = buf_reader.read_line(&mut response).await {

                        if response == committed_res {
                            { // this block forces the mutex to unlock when it goes out of scope so we don't wait for db query before it unlocks
                                let mut mtx_guard = committed_hosts.lock().await;
                                (*mtx_guard).push(host);
                            }
                        }
                    }
                }
            }
        });

    }

    // will need a mechanism here for if we didn't reach redundancy

    Ok(())
}


/* ------------------------------- DB RELATED ------------------------------- */

async fn db_connection() -> Result<Client, Box<dyn Error>> {
    let mut client_options = ClientOptions::parse(DB_CONN_STRING).await?;
    client_options.app_name = Some(String::from("Cortex"));
    let client = Client::with_options(client_options)?;
    Ok(client)
}


async fn query_hosts(args: CortexArgs, db_conn: &Client) -> Result<Cursor<Host>, Box<dyn Error>> {
    let hosts: Collection<Host> = db_conn.database("hosts_db").collection("hosts");
    let filter: Document;
    if args.make == "ANY" {
        filter = doc! {
            "gpus": doc! { "$gte": args.gpus},
            "state": HostState::Available
        };
    } else {
        filter = doc! {
        "gpus": doc! { "$gte": args.gpus},
        "make": args.make.to_uppercase(),
        "available": true
    };
    }

    // for now we return double the redundancy level documents to account for hosts that do not respond
    let hosts_cursor = hosts.find(filter).batch_size(args.redundancy * 2).await?;
    Ok(hosts_cursor)
}

// hosts might need to be passed in as a vector too
async fn alter_batch_state(hosts: Cursor<Host>, new_state: HostState, db_conn: &Client) {
    ()
}

/* -------------------------------------------------------------------------- */




/* ---------------------------------- Tests --------------------------------- */

#[cfg(test)]
mod tests {

}