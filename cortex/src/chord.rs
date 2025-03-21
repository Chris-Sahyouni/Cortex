use tokio::sync::{mpsc::Sender, oneshot::Receiver};

use crate::dispatch::{LocalMessage, LocalQuery, LocalResponse};


/* ------------------------------ Finger Table ------------------------------ */
pub struct FingerTable {
    table: Vec<FingerTableEntry>
}

struct FingerTableEntry {
    position: u128,
    dht_id: u128
}

impl FingerTable {

    pub fn new() -> FingerTable {
        FingerTable {
            table: vec![]
        }
    }

    pub fn fix_fingers(&self) {
        todo!()
    }

}

// this cna be used as a prototype for the other worker threads loops
pub fn chord_manager_loop(to_dispatcher_channel: Sender<LocalMessage>) {
    // create oneshot thread every time to be sent along with the DispatcherQueries for main to send response on
    todo!()
}

// add structs for successor and predecessor if necessary