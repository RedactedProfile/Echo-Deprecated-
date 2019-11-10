extern crate yaml_rust;
extern crate tokio;

use std::fs;
use std::net::SocketAddr;

use yaml_rust::{Yaml, YamlLoader};

use tokio::prelude::*;
use tokio::io;
use tokio::net::TcpListener;

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

fn main() {
    println!("Welcome to Echo\n===============\nGetting config ...");
    let _config = Config::parse();
    println!("Creating server...");
    let _host = _config.get("host");
    let _port = _config.get_int("port");

    let _connection_string:String = format!("{}:{}", &_host, &_port.to_string()).to_string();
    let _connection_addr = _connection_string.parse::<SocketAddr>().unwrap();

    let listener = TcpListener::bind(&_connection_addr)
                                .expect("unable to bind TCP socket, aborting");

    println!("Server running on {}", &_connection_string);
    println!("---------------------------------");

    let server = listener.incoming()
        .map_err(|e| eprintln!("accept failed! = {:?}", e))
        .for_each(|sock| {
            let (reader, writer) = sock.split();

            let bytes_copied = io::copy(reader, writer);

            let msg = bytes_copied.then(|result| {
                match result {
                    Ok((amount, _, _)) => println!("wrote {} bytes", amount),
                    Err(e) => eprintln!("error: {}", e)
                }

                Ok(())
            });

            tokio::spawn(msg)
        });

    tokio::run(server);
}
