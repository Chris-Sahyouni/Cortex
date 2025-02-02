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
// you actually will probably have to do this so that the triggering libraries can serialize messages following this protocol

// the full message to be sent
#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkMessage {
    pub queries: Vec<NetworkQuery>,
    pub responses: Vec<NetworkResponse>
}

// you really need to change the names of all these structs
#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkQuery {
    pub query_id: u32,
    pub cmd: CortexCommand,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkResponse {
    pub query_id: u32,
    pub res: u8 // once again u8 is just a place holder
}

/* ----------------------------- implementations ---------------------------- */

impl NetworkQuery {
    pub fn new(query_id: u32, cmd: CortexCommand) -> NetworkQuery {
        NetworkQuery {
            query_id: query_id,
            cmd: cmd
        }
    }
}

impl NetworkMessage {

    // builder pattern might be better here

    pub fn new() -> NetworkMessage {
        NetworkMessage {
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

    pub fn add_query(&mut self, query: NetworkQuery) {
        self.queries.push(query);
    }

    pub fn add_response(&mut self, res: NetworkResponse) {
        self.responses.push(res);
    }

}