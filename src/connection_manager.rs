use std::net::{TcpListener, TcpStream};
use std::io::Read;
use crate::message::parse;

struct Node(String, String);

pub struct ConnectionManager {
    host: String,
    port: String,
    nodes: Vec<Node>,
}

impl ConnectionManager {
    pub fn new(host: String, port: String) -> Self {
        println!("Initializing connection manager...");
        Self {
            host: host.clone(),
            port: port.clone(),
            nodes: vec![Node(host.clone(), port.clone())]
        }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap();

        for stream in listener.incoming() {
            Self::handle_connection(stream.unwrap());
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 512];

        let size = stream.read(&mut buffer).unwrap();
        let s = String::from_utf8_lossy(&buffer[..size]).to_string();
        let m = parse(&s);

        println!("Message: {:?}", m);
    }
}
