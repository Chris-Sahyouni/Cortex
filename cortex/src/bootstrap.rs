use std::error::Error;
use cortex::{CortexNode, CortexCommand};
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::messaging::net::{NetworkMessage, NetworkQuery, NetworkResponse};
// use messaging::net::{CortexMessage, MessageQuery};



// Asks initial_successor for its successors vector
// initial_successor is the result of introduction
// returns a result whose Ok is the pair (successors, predecessor)
async fn join_network(initial_successor: CortexNode) -> Result<(Vec<CortexNode>, Option<CortexNode>), Box<dyn Error>> {
    let predecessor: Option<CortexNode> = None;

    // if possible, replace this with TLS later
    let mut stream = TcpStream::connect(initial_successor.socket).await?;

    // let msg: CortexMessage = CortexMessage::new();

    // stream.write_all(bincode::serialize(&msg)?.as_slice()).await?;


    todo!()
}