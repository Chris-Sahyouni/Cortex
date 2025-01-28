/* This file defines messaging done over the network between nodes */

use cortex::{CortexNode, CortexCommand};
use serde::{Serialize, Deserialize};
use tokio::net::{TcpListener, TcpStream};
use std::{fmt::Debug, net::SocketAddr, vec};
/* ------------------------- Cortex Network Protocol ------------------------ */
/*
    CortexMessage represents the full message to be sent to another node. It is built by the dispatcher which will
    receive pieces from other threads, aggregate them into a single message and then send them to their destinations.

    DispatcherQueries are messages to be sent from various threads to the dispatcher which will then be parsed and added to the next
    outgoing CortexMessage. Likewise, upon receiving messages from other nodes, DispatcherResponses will be sent to their respective threads.

    MessageQuery and MessageResponse are basically the same as DispatcherQuery and DispatcherResponse but for messages actually going between nodes not threads

*/


const CORTEX_PORT: u16 = 32503;



// for performance you could create your own serializer and deserializer

// the full message to be sent
#[derive(Serialize, Deserialize, Debug)]
pub struct CortexMessage {
    pub queries: Vec<MessageQuery>,
    pub responses: Vec<MessageResponse>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageQuery {
    pub query_id: u32,
    pub cmd: CortexCommand
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    pub query_id: u32,
    pub res: u8 // once again u8 is just a place holder
}

/* ----------------------------- implementations ---------------------------- */

impl CortexMessage {

    // builder pattern might be better here

    pub fn new() -> CortexMessage {
        CortexMessage {
            queries: vec![],
            responses: vec![]
        }
    }

    pub fn send(&self, dst: CortexNode) {
        todo!()
    }

    pub fn multicast(&self, dsts: Vec<CortexNode>) {
        todo!()
    }

    pub fn add_query(&mut self, query: MessageQuery) {
        self.queries.push(query);
    }

    pub fn add_response(&mut self, res: MessageResponse) {
        self.responses.push(res);
    }

}