use std::{collections::HashMap, net::SocketAddr, process::{self}, vec};
use tokio::{self, sync::{mpsc, oneshot}};
use rand::Rng;
use cortex::{get_dht_id, CortexCommand, CortexNode, DestinationType};

mod messaging;
use messaging::net::{NetworkMessage, NetworkQuery};


mod bootstrap;

mod dispatch;
use dispatch::{parse_network_messages, parse_worker_thread_messages, LocalMessage, LocalQuery, LocalResponse, ThreadRoutingTable};

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

    let mut thread_routing_table: ThreadRoutingTable = ThreadRoutingTable::new();

    // these will be initialized by bootstrap so these are just placeholders
    let successors: Vec<CortexNode> = vec![];
    let predecessor: CortexNode = CortexNode {dht_id: 0, socket: SocketAddr::new(local_ip_address::local_ip().unwrap(), 0)}; // placeholder
    let finger_table: FingerTable;

    // Create thread communication channel

    // This one lets all threads send queries to the dispatcher, dispatcher will receive them on rcv_query_channel
    let (to_dispatcher_channel, mut from_worker_threads_channel): (mpsc::Sender<LocalMessage>, mpsc::Receiver<LocalMessage>) = mpsc::channel(256);

    // This type of of channel will have to be created for each worker thread
    // This one is just a prototype
    let (send_res_channel, recv_res_channel): (oneshot::Sender<LocalMessage>, oneshot::Receiver<LocalMessage>) = oneshot::channel();

    // Create worker threads
    let channel_for_chord_manager = to_dispatcher_channel.clone();
    let chord_manager_thread_handler = tokio::spawn(async {
        chord_manager_loop(channel_for_chord_manager);
    });


    // Main loop
    loop {
        /* main loop steps
            UPDATED VERSION
            1. Receive (both queries and responses)
                - from worker threads
                - from network

                    - this happens on mpsc recv side
            2. Update
                - add queries to table
                - build outgoing messages out of queries/responses from worker threads
            3. Send
                - Send queries/responses from network to worker threads
                - send network messages to their destinations

            this means the mpsc channels for the dispatcher to receive on remains

        */

        // RECEIVE

        // receive messages from worker threads
        let mut incoming_worker_thread_msgs: Vec<LocalMessage> = vec![];
        from_worker_threads_channel.recv_many(&mut incoming_worker_thread_msgs, 32).await;

        // separate worker thread messages into queries and responses
        let (local_queries, local_responses) = parse_worker_thread_messages(incoming_worker_thread_msgs);

        // receive messages from the network
        let mut incoming_network_msgs: Vec<NetworkMessage> = vec![]; // placeholder, will be populated by streams in the future
        let (network_queries, network_responses) = parse_network_messages(&mut incoming_network_msgs);

        // UPDATE
        thread_routing_table.add_from_worker_queries(local_queries);
        


        // map of destination to outgoing message
        let mut outgoing_messages: HashMap<u128, NetworkMessage> = HashMap::new();


        // for query in incoming_worker_thread_msgs {
        //     if thread_routing_table.contains(query.query_id) {
        //         // for debugging purposes
        //         println!("For some reason a query_id was already in the query table. This shouldn't happen");
        //     } else {
        //         // normal case

        //         // if a response is required, add it to the query table
        //         if let Some(send_response_channel) = query.opt_send_response_channel {
        //             thread_routing_table.add_entry(query.query_id, send_response_channel);
        //         }

        //         // add the message to the relevant CortexMessages
        //         match query.dst {
        //             DestinationType::Successors => {
        //                 for successor in &successors {
        //                     if let Some(out_msg) = outgoing_messages.get_mut(&successor.dht_id) {
        //                         out_msg.add_query(NetworkQuery::new(query.query_id, query.cmd));
        //                     } else {
        //                         // CortexMessage was not in hash table yet, create and add
        //                         let mut out_msg = NetworkMessage::new();
        //                         out_msg.add_query(NetworkQuery::new(query.query_id, query.cmd));
        //                         outgoing_messages.insert(successor.dht_id, out_msg);
        //                     }
        //                 }
        //             }
        //             DestinationType::Predecessor => {
        //                 if let Some(out_msg) = outgoing_messages.get_mut(&predecessor.dht_id) {
        //                     out_msg.add_query(NetworkQuery::new(query.query_id, query.cmd));
        //                 } else {
        //                     // CortexMessage was not in hash table yet, create and add
        //                     let mut out_msg = NetworkMessage::new();
        //                     out_msg.add_query(NetworkQuery::new(query.query_id, query.cmd));
        //                     outgoing_messages.insert(predecessor.dht_id, out_msg);
        //                 }
        //             }
        //             DestinationType::Single(target_node) => {
        //                 if let Some(out_msg) = outgoing_messages.get_mut(&target_node.dht_id) {
        //                     out_msg.add_query(NetworkQuery::new(query.query_id, query.cmd));
        //                 } else {
        //                     // CortexMessage was not in hash table yet, create and add
        //                     let mut out_msg = NetworkMessage::new();
        //                     out_msg.add_query(NetworkQuery::new(query.query_id, query.cmd));
        //                     outgoing_messages.insert(target_node.dht_id, out_msg);
        //                 }
        //             }
        //         }

        //     }
        // }



        // I think it would better if this actually happened first




        let mut fresh_responses: Vec<LocalResponse> = vec![];

    }

}
