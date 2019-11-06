extern crate yaml_rust;
use std::io;
use std::fs;
use std::net::{TcpListener, TcpStream};
use yaml_rust::{YamlLoader};



fn handle_client(_stream: TcpStream) {
    println!("hit")
}

fn get_config (value:String) -> String {
    let _config_file:String = fs::read_to_string("./conf.yaml").unwrap();
    let _config = YamlLoader::load_from_str(&_config_file).unwrap();

    let doc= &_config[0];

    return doc["server"]["host"].as_str().unwrap().parse().unwrap();
}

fn parse_config() {
    let get = get_config;
    (get);
}

fn main() -> io::Result<()> {
    let _config = parse_config();
    println!("{:?}", _config().get("host"));

    let listener = TcpListener::bind("0.0.0.0:9875")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
