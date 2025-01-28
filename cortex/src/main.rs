use std::{process::{self}, vec};
use tokio::{self, sync::{mpsc, oneshot}};
use rand::Rng;
use cortex::{get_dht_id, CortexCommand, CortexNode};

mod messaging;
use messaging::{net::{CortexMessage, MessageQuery, MessageResponse}};


mod bootstrap;

mod dispatch;
use dispatch::{QueryTable, DispatcherQuery, DispatcherResponse};

mod chord;
use chord::{FingerTable, chord_manager_loop};

/* ORDER OF EVENTS
    1. Cortex process starts up
      - check for cached successor/predecessor on disk to avoid contacting introducer
      - if found, contact directly, else contact introducer
    2. Get added to the network once successor is determined
      - ask successor for its successors vector, take all but last element
      - predecessor should be found in stabilize
      - begin heartbeating and stabilizing
    3. Initialize finger table
    4. Acquire keys
    5. Good to go
*/

/* MAIN.RS WILL ALSO ACT AS THE DISPATCHER FOR NOW */


const BATMAN_QUOTES: [&str; 4] = [
    "The shadows betray you because they belong to me",
    "The training is nothing. The will is everything",
    "Victory has defeated you",
    "Save yourself. You don't owe these people anymore. You've given them everything. \n Not Everything... Not yet"
];

// const NUM_SUCCESSORS: u8 = 3;

#[tokio::main]
async fn main() {

    // Drop batman quote to increase happiness
    let mut rng = rand::thread_rng();
    let quote_idx = rng.gen_range(0..=3);
    println!("{}", BATMAN_QUOTES[quote_idx]);

    // Get DHT Id
    // There's a problem, how will we identify nodes for payment, ip can change and so therefore so can dht_id
    //   Maybe hash MAC address instead?
    let dht_id: u128;
    let get_dht_id_res = get_dht_id();
    match get_dht_id_res {
        Ok(id) => { dht_id = id }
        Err(msg) => {
            println!("{}", msg);
            process::exit(1)
        }
    }
    println!("My DHT Id: {}", dht_id);


    /* Here for now we will assume that there is a known other node for this one to contact.
       Later we can implement the logic for contacting an introducer and actually getting this information. */

    /* Joining the network will also be left until later, it will be easier to implement once the behavior
       of the nodes in the network is defined */

    // piggybacking should not happen until the node is in the network, too complicated otherwise

    // let query_table = Arc::new(Mutex::new(HashMap<))

    let mut query_table: QueryTable = QueryTable::new();

    // these will be initialized by bootstrap
    let successors: Vec<CortexNode>;
    let predecessor: CortexNode;
    let finger_table: FingerTable;

    // Create thread communication channel

    // This one lets all threads send queries to the dispatcher, dispatcher will receive them on rcv_query_channel
    let (send_query_channel, mut rcv_query_channel): (mpsc::Sender<DispatcherQuery>, mpsc::Receiver<DispatcherQuery>) = mpsc::channel(256);

    // This type of of channel will have to be created for each worker thread
    // This one is just a prototype
    let (send_res_channel, recv_res_channel): (oneshot::Sender<DispatcherResponse>, oneshot::Receiver<DispatcherResponse>) = oneshot::channel();

    // Create worker threads
    let send_query_channel_chord_manager = send_query_channel.clone();
    let chord_manager_thread_handler = tokio::spawn(async {
        chord_manager_loop(send_query_channel_chord_manager);
    });


    // Main loop
    loop {
        /* main loop steps
            1. receive DispatcherQueries from worker threads
            2. update query_table accordingly
            3. construct CortexMessages
            4. send CortexMessges
            5. receive CortexMessages
            6. parse into responses for worker threads
            7. deliver responses to worker threads
            8. wait a brief heartbeat period
         */



        // receive DispatcherQueries from worker threads
        let mut fresh_queries: Vec<DispatcherQuery> = vec![];
        rcv_query_channel.recv_many(&mut fresh_queries, 32).await;

        // update query_table and construct CortexMessage
        let outgoing_message = CortexMessage::new();
        for query in fresh_queries {
            if query_table.contains(query.query_id) {
                // for debugging purposes
                println!("For some reason a query_id was already in the query table. This shouldn't happen");
            } else {
                // normal case
                query_table.add_entry(query.query_id, query.send_response_channel);
            }
        }

        // Logging off for now
        // YOU NEED A WAY TO HANDLE RESPONSES TOO, HOW ARE THESE QUERIES ACTUALLY GOING TO BE HANDLED?




        let mut fresh_responses: Vec<DispatcherResponse> = vec![];

    }

}
