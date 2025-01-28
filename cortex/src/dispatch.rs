use std::collections::HashMap;
use cortex::{CortexCommand, CortexNode};
use tokio::sync::oneshot;

// I don't think the query table is going to be accessed by any thread other than the main/dispatcher thread
// so as of now I don't think a mutex or Arc is necessary


pub struct QueryTable {
    table: HashMap<u32, QueryTableEntry>
}

struct QueryTableEntry {
    send_response_channel: oneshot::Sender<DispatcherResponse>,
    age: u16
}

// right now this is literally just a wrapper on a Hashmap. Just delete the entire thing if no other functionality is ever necessary.
impl QueryTable {

    pub fn new() -> QueryTable {
        QueryTable {
            table: HashMap::new()
        }
    }

    pub fn add_entry(&mut self, query_id: u32, send_res_chan: oneshot::Sender<DispatcherResponse>) {
        self.table.insert(query_id, QueryTableEntry { send_response_channel: send_res_chan, age: 0 });
    }

    pub fn remove_entry(&mut self, query_id: u32) {
        self.table.remove(&query_id);
    }

    pub fn contains(&self, query_id: u32) -> bool {
        self.table.contains_key(&query_id)
    }
}

#[derive(Debug)]
pub struct DispatcherQuery {
    pub query_id: u32,
    pub send_response_channel: oneshot::Sender<DispatcherResponse>,
    pub dst: CortexNode,
    pub cmd: CortexCommand
}

#[derive(Debug)]
pub struct DispatcherResponse {
    pub query_id: u32,
    pub res: u8 // this type should probably be a trait later, u8 is just a placeholder for now
}