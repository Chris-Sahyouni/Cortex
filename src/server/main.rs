use std::{collections::hash_map::DefaultHasher, error::Error, io::prelude::*, mem::take, sync::{mpsc::channel, Arc}};
use clap::builder::styling::Color;
use cortex::{CortexArgs, CortexCommands, JobState};
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::{TcpListener, TcpStream}, sync::Mutex};
use futures::{stream::StreamExt, TryStreamExt};

mod host;
use host::{Host, HostState};

extern crate mongodb;
use mongodb::{bson::{doc, Document}, options::ClientOptions, Client, Collection, Cursor};

extern crate cuckoofilter;
use cuckoofilter::CuckooFilter;

const SERVER_ADDR: &str = "127.0.0.1:32503";
const DB_CONN_STRING: &str = "mongodb://localhost:27017";


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(SERVER_ADDR).await.unwrap();
    // let (sender, receiver) = channel();

    let db_conn = db_connection().await.unwrap();

    let working_host_ids = Arc::new(Mutex::new(CuckooFilter::new()));

    loop {

        // shadow both these values to create new reference counting pointers to be moved into each future

        // uses Arc internally, so its fine to clone db_conn in a loop
        let db_conn = db_conn.clone();
        let working_host_ids = working_host_ids.clone();

        // connection attempt may not be successful so we pattern match it to ensure its Ok
        if let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(async move {
                if let Ok(()) = handle_connection(stream, &db_conn, working_host_ids).await {
                    println!("task succeeded");
                }
            });
        }
    }
}


async fn handle_connection(mut stream: TcpStream, db_conn: &Client, working_host_ids: Arc<Mutex<CuckooFilter<DefaultHasher>>>) -> Result<(), Box<dyn Error + Send + Sync>> {

    // serialize the bytes from the stream into a CortexArgs
    let mut buf_reader = BufReader::new(&mut stream);
    let mut serialized_args = String::new();
    buf_reader.read_line(&mut serialized_args).await?;
    let args: CortexArgs = serde_json::from_str(serialized_args.as_str()).unwrap();

    // query the db
    let mut hosts_cursor = query_hosts(args.clone(), &db_conn).await?;

    // if two phase commit succeeds, alter the committed host states
    if let Ok(committed_hosts) = two_phase_commit(&mut hosts_cursor, args.redundancy.clone(), args.id.clone(), &db_conn, working_host_ids.clone()).await {
        alter_batch_state(&committed_hosts, HostState::Working(args.id), &db_conn).await;

    }

    Ok(())
}

async fn two_phase_commit(cursor: &mut Cursor<Host>, redundancy: u32, job_id: String, db_conn: &Client, working_host_ids: Arc<Mutex<CuckooFilter<DefaultHasher>>>) -> Result<Vec<Host>, Box<dyn Error + Send + Sync>> {

    let committed_hosts: Arc<Mutex<Vec<Host>>> = Arc::new(Mutex::new(Vec::new()));

    while let Some(Ok(host)) = cursor.next().await {

        {
            let num_committed_hosts = committed_hosts.lock().await.len() as u32;
            if num_committed_hosts >= redundancy {
                break;
            }
        }

        let working_host_ids = working_host_ids.clone();
        { // this block should fix the race condition where a host was "available" in the db but committed to another job first
            let mut working_host_ids = working_host_ids.lock().await;
            if working_host_ids.contains(&host.id) {
                continue;
            }
            if let Err(_) = working_host_ids.add(&host.id) {
                println!("Error adding host to the cuckoo filter");
                continue;
            }
        }

        // make copies of anything that will be needed after being moved into the future
        let commit_req = String::from("Commit-Request:") + job_id.as_str() + "\n";
        let committed_res = String::from("Committed:") + job_id.as_str() + "\n";
        let committed_hosts = Arc::clone(&committed_hosts);
        let host_id = host.id.clone();

        tokio::spawn(async move {
            let host_socket = host.ip.to_string() + ":32503";
            let mut remove_from_cf = true;
            if let Ok(mut stream) = TcpStream::connect(host_socket).await {
                if let Ok(()) = stream.write_all(commit_req.as_bytes()).await {
                    let mut buf_reader = BufReader::new(&mut stream);
                    let mut response = String::new();
                    if let Ok(_bytes_read) = buf_reader.read_line(&mut response).await {

                        if response == committed_res {
                            { // this block forces the mutex to unlock when it goes out of scope so we don't wait to add to the cuckoo filter before unlocking
                                let mut committed_hosts = committed_hosts.lock().await;
                                committed_hosts.push(host);
                                remove_from_cf = false;
                            }
                        }
                    }
                }
            }
            if remove_from_cf {
                let mut working_host_ids = working_host_ids.lock().await;
                working_host_ids.delete(&host_id);
            }
        });
    }

    // will need a mechanism here for if we didn't reach redundancy

    let mut committed_hosts = committed_hosts.lock().await;
    Ok(take(&mut *committed_hosts))
}


/* ------------------------------- DB RELATED ------------------------------- */

async fn db_connection() -> Result<Client, Box<dyn Error + Send + Sync>> {
    let mut client_options = ClientOptions::parse(DB_CONN_STRING).await?;
    client_options.app_name = Some(String::from("Cortex"));
    let client = Client::with_options(client_options)?;
    Ok(client)
}


async fn query_hosts(args: CortexArgs, db_conn: &Client) -> Result<Cursor<Host>, Box<dyn Error + Send + Sync>> {
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
        "state": HostState::Available
    };
    }

    // for now we return double the redundancy level documents to account for hosts that do not respond
    let hosts_cursor = hosts.find(filter).batch_size(args.redundancy * 2).await?;
    Ok(hosts_cursor)
}

// alters all given hosts to new_state
async fn alter_batch_state(host_ids: &Vec<Host>, new_state: HostState, db_conn: &Client) {
    let hosts_collection: Collection<Host> = db_conn.database("hosts_db").collection("hosts");
    for host in host_ids {
        let query = doc! {"id": host.id.clone()};
        let update = doc! {"state": new_state.clone()};
        let _res = hosts_collection.update_one(query, update).await; // should add error handling here if update_one fails
    }
}

/* -------------------------------------------------------------------------- */




/* ---------------------------------- Tests --------------------------------- */

#[cfg(test)]
mod tests {

}