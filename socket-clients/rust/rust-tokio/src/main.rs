use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWrite;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let mut stream = TcpStream::connect("172.32.0.3:60401").await?;
    let subscribe = "{\"method\":\"blockchain.headers.subscribe\",\"params\":[],\"id\":90003}\r\n".to_string();
    stream.write_all(&subscribe.as_bytes()).await?;

    loop {
        let mut buf = [0; 1024];
        let n = stream.read(&mut buf).await?;
        let recieved = String::from_utf8(buf[0..n].to_vec())?;
        println!("They sent: {:?}", recieved);
    }

    Ok(())
}
