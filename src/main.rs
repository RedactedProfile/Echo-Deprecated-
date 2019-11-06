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
}

fn handle_client(_stream: TcpStream) {
    println!("hit")
}

fn main() -> io::Result<()> {
    let config = Config::parse();
    let host = config.get("host");

    println!("{:?}", host);

    let listener = TcpListener::bind("0.0.0.0:9875")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
