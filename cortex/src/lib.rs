use std::{error::Error, net::SocketAddr};

use local_ip_address::local_ip;
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct CortexNode {
    pub dht_id: u128,
    pub socket: SocketAddr,
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