
use cortex::CortexNode;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;


/* ------------------------- Cortex Network Protocol ------------------------ */
/*
    CortexMessage represents the full message to be sent to another node. It is built by the dispatcher which will
    receive pieces from other threads, aggregate them into a single message and then send them to their destinations.

    DispatcherQueries are messages to be sent from various threads to the dispatcher which will then be parsed and added to the next
    outgoing CortexMessage. Likewise, upon receiving messages from other nodes, DispatcherResponses will be sent to their respective threads.

    MessageQuery and MessageResponse are basically the same as DispatcherQuery and DispatcherResponse but for messages actually going between nodes not threads

*/








// for performance you could create your own serializer and deserializer

// the full message to be sent
#[derive(Serialize, Deserialize, Debug)]
pub struct CortexMessage {
    pub hearbeat: bool,
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


#[derive(Debug)]
pub struct DispatcherQuery {
    pub query_id: u32,
    // src: String, // src shouldn't actually be necessary, the thread is the src
    pub dst: CortexNode,
    pub cmd: CortexCommand
}

#[derive(Debug)]
pub struct DispatcherResponse {
    pub query_id: u32,
    pub res: u8 // this type should probably be a trait later, u8 is just a placeholder for now
}

// all possible commands enumerated
#[derive(Serialize, Deserialize, Debug)]
pub enum CortexCommand {
    SendSuccessors
}

impl CortexMessage {

    // builder pattern might be better here

    pub fn new(queries: Vec<MessageQuery>, responses: Vec<MessageResponse>) -> CortexMessage {
        CortexMessage {
            hearbeat: false,
            queries: queries,
            responses: responses
        }
    }

    pub fn send_message(&self) {
        todo!()
    }

}