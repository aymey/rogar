use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use protocol::modern::*;
use std::mem::*;

fn handle_client(mut stream: TcpStream) {
    const SIZE: usize = size_of::<ReqProtocolVersion>();
    let mut connection = [0 as u8; SIZE];

    stream.read_exact(&mut connection).unwrap();
    let player = unsafe {transmute::<[u8; SIZE], ReqProtocolVersion>(connection)};
    println!("{player:?}");
}

fn main() -> std::io::Result<()> {
    let listner = TcpListener::bind("0.0.0.0:8080")?;

    for stream in listner.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
