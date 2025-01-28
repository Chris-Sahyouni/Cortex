use std::{error::Error, net::SocketAddr};

/* This file defines our own custom stream types to simplify network connections */
use easy_tokio_rustls::{TlsClient, TlsServer, TlsStream};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

// const KEY_FILE: &str = "<placeholder for key file>";
// const CERT_FILE: &str = "<placeholder for cert file>";
// const CORTEX_PORT: &str = "32503";
const BUFFER_SIZE: usize = 1028;


// a wrapper on TLS streams to simplify usage
pub struct CortexStream {
    connection: TlsStream<TcpStream>,
    // buffer: [u8; BUFFER_SIZE]
}

impl CortexStream {

    pub async fn connect(dst: SocketAddr) -> Result<CortexStream, Box<dyn Error>> {
        let conn = TlsClient::new(dst).await?.connect().await?;
        // let buff: [u8; BUFFER_SIZE];
        Ok(CortexStream {
            connection: conn,
            // buffer: buff
        })
    }

    pub async fn write(&self) {
        todo!()
    }

    pub fn read(&self) {
        todo!()
    }
}

