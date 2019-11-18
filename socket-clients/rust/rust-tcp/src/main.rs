use std::io::prelude::*;
use std::io::{self, Read};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("172.32.0.3:60401")?;
    stream.set_nonblocking(true).expect("set_nonblocking call failed");

    let s = "{\"method\":\"server.banner\",\"params\":[],\"id\":90003}\r\n".to_string(); 
    stream.write(&s.as_bytes())?;

    stream.read(&mut [0; 128])?;
    Ok(())
}
