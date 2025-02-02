use std::{error::Error, net::SocketAddr};
use serde::{Serialize, Deserialize};
use local_ip_address::local_ip;
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize)]
pub struct CortexNode {
    pub dht_id: u128,
    pub socket: SocketAddr,
}

pub fn consistent_hash(data: impl AsRef<[u8]>) -> u128 {
    let mut hasher = Sha256::new();
    hasher.update(data);
    // dht_id_string = format!("{:x}", hasher.finalize()); // formats hash as hex string
    let full_hash_bytes: [u8; 32] = hasher.finalize().as_slice().try_into().expect("There is never a reason converting the hash output to [u8; 32] should fail");

    /* Why cut the hash in half?
    SHA-256 outputs 256 bits but Rust's largest integer type is only 128 bits.
    Cutting the hash in half just makes it so much simpler to work with because
    we can use u128 as the type for our hash instead of big-integer which is
    so much simpler to work with.
    */
    let half_hash_bytes: [u8; 16] = full_hash_bytes[..16].try_into().expect("There is never a reason cutting the hash in half should fail");

    let half_hash_bytes_u128: u128 = u128::from_be_bytes(half_hash_bytes).try_into().expect("There is never a reason converting an array of 16 u8's into a u128 should fail");
    half_hash_bytes_u128 % (2^128)
}

// Hash the local ip address to compute this node's DHT Id
pub fn get_dht_id() -> Result<u128, Box<dyn Error>> {
    if let Ok(ip) = local_ip() {
        let dht_id = consistent_hash(ip.to_string());
        Ok(dht_id)
    } else {
        Err("Failed to fetch local ip address".into()) // .into() simply wraps the string in Box<dyn Error>
    }
}

// all possible commands enumerated
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum CortexCommand {
    SendSuccessors,
    Heartbeat,
    Forward
}

#[derive(Debug)]
pub enum DestinationType {
    Single(CortexNode),
    Successors,
    Predecessor,
}