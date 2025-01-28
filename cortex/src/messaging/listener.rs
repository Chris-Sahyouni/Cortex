use std::error::Error;

use easy_tokio_rustls::{TlsListener, TlsServer};
use local_ip_address::local_ip;

const CORTEX_PORT: &str = "32503";
const KEY_FILE: &str = "<placeholder for key file>";
const CERT_FILE: &str = "<placeholder for cert file>";

pub struct CortexListener {
    listener: TlsListener
}

impl CortexListener {

    pub async fn new() -> Result<CortexListener, Box<dyn Error>> {
                // pub async fn listen() -> Result<CortexStream, Box<dyn Error>> {
        let ip = local_ip()?.to_string();
        let socket = ip + ":" + CORTEX_PORT;
        let listener: TlsListener = TlsServer::new(socket, CERT_FILE, KEY_FILE).await?.listen().await?;
        Ok(CortexListener {
            listener: listener
        })
    }

}