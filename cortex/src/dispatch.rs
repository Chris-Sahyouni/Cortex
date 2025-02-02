use std::{collections::HashMap, vec};
use cortex::{CortexCommand, CortexNode, DestinationType};
use tokio::sync::oneshot;

use crate::messaging::net::{NetworkMessage, NetworkQuery, NetworkResponse};


// I don't think the query table is going to be accessed by any thread other than the main/dispatcher thread
// so as of now I don't think a mutex or Arc is necessary


pub struct ThreadRoutingTable {
    table: HashMap<u32, Entry>,
}

// query came from the worker, response will be sent back
struct FromWorkerEntry {
    to_worker_channel: oneshot::Sender<LocalResponse>,
    age: u16
}

// query was sent to the worker, response will be received from it
struct ToWorkerEntry {
    from_worker_channel: oneshot::Receiver<LocalResponse>,
    age: u16
}

enum Entry {
    FromWorker(FromWorkerEntry),
    ToWorker(ToWorkerEntry)
}

// right now this is literally just a wrapper on a Hashmap. Just delete the entire thing if no other functionality is ever necessary.
impl ThreadRoutingTable {

    pub fn new() -> ThreadRoutingTable {
        ThreadRoutingTable {
            table: HashMap::new()
        }
    }

    fn add_entry(&mut self, query_id: u32, entry: Entry) {
        self.table.insert(query_id, entry);
    }

    fn remove_entry(&mut self, query_id: u32) {
        self.table.remove(&query_id);
    }

    pub fn contains(&self, query_id: u32) -> bool {
        self.table.contains_key(&query_id)
    }

    pub fn add_from_worker_queries(&mut self, queries: Vec<LocalQuery>) {
        for query in queries {
            // only add it to the table if a response is required
            if let Some(to_worker_channel) = query.opt_send_response_channel {
                let from_worker_entry: FromWorkerEntry = FromWorkerEntry {
                    to_worker_channel: to_worker_channel,
                    age: 0
                };
                self.add_entry(query.query_id, Entry::FromWorker(from_worker_entry));
            }
        }
    }
}

#[derive(Debug)]
pub struct LocalQuery {
    pub query_id: u32,
    pub opt_send_response_channel: Option<oneshot::Sender<LocalResponse>>, // Some() implies a response is required, None() implies no response is needed
    pub cmd: CortexCommand,
    pub dst: DestinationType,
}

#[derive(Debug)]
pub struct LocalResponse {
    pub query_id: u32,
    pub res: u8 // this type should probably be a trait later, u8 is just a placeholder for now
}

// having this enum to wrap Query and Message
#[derive(Debug)]
pub enum LocalMessage {
    Query(LocalQuery),
    Response(LocalResponse)
}

pub fn parse_network_messages(msgs: &mut Vec<NetworkMessage>) -> (Vec<NetworkQuery>, Vec<NetworkResponse>) {
    let mut network_queries: Vec<NetworkQuery> = vec![];
    let mut network_responses: Vec<NetworkResponse> = vec![];
    for msg in msgs {
        network_queries.append(&mut msg.queries);
        network_responses.append(&mut msg.responses);
    }
    (network_queries, network_responses)
}

pub fn parse_worker_thread_messages(msgs: Vec<LocalMessage>) -> (Vec<LocalQuery>, Vec<LocalResponse>) {
    let mut local_queries: Vec<LocalQuery> = vec![];
    let mut local_responses: Vec<LocalResponse> = vec![];
    for msg in msgs {
        match msg {
            LocalMessage::Query(q) => {
                local_queries.push(q);
            }
            LocalMessage::Response(r) => {
                local_responses.push(r);
            }
        }
    }
    (local_queries, local_responses)
}
