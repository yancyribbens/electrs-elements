use std::io::prelude::*;
use std::io::{self, Read, BufReader};
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("172.32.0.3:60401")?;
    stream.set_nonblocking(true).expect("set_nonblocking call failed");

    let banner = "{\"method\":\"server.banner\",\"params\":[],\"id\":90003}\r\n".to_string(); 
    let version = "{\"method\":\"server.version\",\"params\":[],\"id\":90003}\r\n".to_string();
    let subscribe = "{\"method\":\"blockchain.headers.subscribe\",\"params\":[],\"id\":90003}\r\n".to_string();

    stream.write(&subscribe.as_bytes())?;

    let mut line;
    loop {
	line = [0; 512];
	let result = stream.read(&mut line);
	match result {
	    Ok(n) => {
                if n > 0 {
                    println!("{}: Received: {}", n, str::from_utf8(&line).unwrap());
                }
            },
	    _ => { },
	};
    }

    Ok(())
}
