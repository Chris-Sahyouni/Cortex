use std::{error::Error, process::{self}};
use tokio;
use local_ip_address::{local_ip};
use sha2::{Digest, Sha256};
use rand::{Rng};

const BATMAN_QUOTES: [&str; 3] = [
    "The shadows betray you because they belong to me",
    "The training is nothing. The will is everything",
    "Victory has defeated you"
];

#[tokio::main]
async fn main() {

    // Drop batman quote to increase happiness
    let mut rng = rand::thread_rng();
    let quote_idx = rng.gen_range(0..=2);
    println!("{}", BATMAN_QUOTES[quote_idx]);

    // Get DHT Id
    let dht_id: u128;
    let get_dht_id_res = get_dht_id();
    match get_dht_id_res {
        Ok(id) => { dht_id = id }
        Err(msg) => {
            println!("{}", msg);
            process::exit(1)
        }
    }

    // Determine whether or not this node is an introducer
    let introducer_freq = 12; // this determines what fraction of all nodes are introducers calculated by 1 / introducer_freq
    if dht_id % introducer_freq == 0 {
        // this node IS an introducer, handle accordingly
        println!("This node is an introducer");
    }

    println!("My DHT Id: {}", dht_id);


}


// Hash the local ip address to compute this node's DHT Id
fn get_dht_id() -> Result<u128, Box<dyn Error>> {
    if let Ok(ip) = local_ip() {
        let mut hasher = Sha256::new();
        hasher.update(ip.to_string());
        // dht_id_string = format!("{:x}", hasher.finalize()); // formats hash as hex string
        let dht_id_bytes: [u8; 32] = hasher.finalize().as_slice().try_into()?;

        /* Why cut the hash in half? SHA-256 outputs 256 bits but Rust's largest integer type is only 128 bits.
        Cutting the hash in half just makes it so much simpler to work with because no big-integer library is
        necessary then. */
        let half_bytes: [u8; 16] = dht_id_bytes[..16].try_into()?;

        let dht_id: u128 = u128::from_be_bytes(half_bytes).try_into()?;
        Ok(dht_id)
    } else {
        Err("Failed to fetch local ip address".into()) // .into() simply wraps the string in Box<dyn Error>
    }
}