extern crate yaml_rust;

use std::fs;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

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

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(b"TESTING").unwrap();
            stream.write(&data[0..size]).unwrap();
            true 
        },
        Err(_) => {
            eprintln!("an error occured, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false 
        }
    } {}
}

fn main() {
    println!("Welcome to Echo\n===============\nGetting config ...");
    let _config = Config::parse();
    println!("Creating server...");
    let _host = _config.get("host");
    let _port = _config.get_int("port");

    let _connection_string:String = format!("{}:{}", &_host, &_port.to_string()).to_string();
    let listener = TcpListener::bind(&_connection_string).unwrap();
    println!("Server running on {}", &_connection_string);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection from {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream);
                });
            },
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    drop(listener);
}
