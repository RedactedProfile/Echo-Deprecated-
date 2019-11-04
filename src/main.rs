use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::Read;

fn handle_client(stream: TcpStream) {
    println!("hit")
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9875")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
