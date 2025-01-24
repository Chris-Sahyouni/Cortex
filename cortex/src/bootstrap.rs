use std::error::Error;
use cortex::CortexNode;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use crate::cnp::{CortexCommand, CortexMessage, MessageQuery};



// Asks initial_successor for its successors vector
// initial_successor is the result of introduction
// returns a result whose Ok is the pair (successors, predecessor)
async fn join_network(initial_successor: CortexNode) -> Result<(Vec<CortexNode>, Option<CortexNode>), Box<dyn Error>> {
    let predecessor: Option<CortexNode> = None;

    // if possible, replace this with TLS later
    let mut stream = TcpStream::connect(initial_successor.socket).await?;

    let msg: CortexMessage = CortexMessage::new(vec![MessageQuery {query_id: 1, cmd: CortexCommand::SendSuccessors}], vec![]);

    stream.write_all(bincode::serialize(&msg)?.as_slice()).await?;
    

    todo!()
}