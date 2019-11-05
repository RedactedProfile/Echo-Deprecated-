use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::Read;
use std::collections::BTreeMap;

struct Configuration {
    host: &'static str,
    port: i8
}

fn parse_config() {
    let new_config: Configuration = Configuration { host: "127.0.0.1", port: 9875 };
    let s = serde_yaml::to_string(&new_config)?;
    print!(s)
}

fn handle_client(stream: TcpStream) {
    println!("hit")
}

fn main() -> io::Result<()> {
    let config = parse_config();

    let listener = TcpListener::bind("0.0.0.0:9875")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
