extern crate yaml_rust;
use std::io;
use std::fs;
use std::net::{TcpListener, TcpStream};
use yaml_rust::{Yaml, YamlLoader};

struct Config {
    doc: Yaml
}

impl Config {
    fn parse() -> Self {
        let config_file = fs::read_to_string("./conf.yaml").unwrap();
        let config = YamlLoader::load_from_str(&config_file).unwrap();

        let doc = config.into_iter().next().expect("empty yaml");
    
        Self { doc }
    }
    
    fn get(&self, value: &str) -> &str {
        self.doc["server"][value]
            .as_str()
            .expect("value not a string")
    }

    fn get_int(&self, value: &str) -> i64 {
        self.doc["server"][value]
            .as_i64()
            .expect("value not a string")
    }
}

fn handle_client(_stream: TcpStream) {
    println!("hit")
}

fn main() -> io::Result<()> {
    println!("Welcome to Echo\n===============\nGetting config ...");
    let _config = Config::parse();
    println!("Creating server...");
    let _host = _config.get("host");
    let _port = _config.get_int("port");

    let _connection_string:String = format!("{}:{}", &_host, &_port.to_string()).to_string();
    let listener = TcpListener::bind(&_connection_string)?;
    println!("Server running on {}", &_connection_string);
    println!("---------------------------------");

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
