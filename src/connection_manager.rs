use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use crate::message::{parse, Type, Message};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Node(pub String, pub String);

pub struct ConnectionManager {
    host: String,
    pub port: String,
    nodes: HashSet<Node>,
}

impl ConnectionManager {
    pub fn new(host: String, port: String) -> Self {
        println!("Initializing connection manager...");

        let mut c = Self {
            host: host.clone(),
            port: port.clone(),
            nodes: HashSet::new()
        };
        c.nodes.insert(Node(host.clone(), port.clone()));
        c
    }

    pub fn start(&mut self) {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap();

        for stream in listener.incoming() {
            self.handle_connection(stream.unwrap());
        }
    }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        let mut buffer = [0; 512];

        let size = stream.read(&mut buffer).unwrap();
        let s = String::from_utf8_lossy(&buffer[..size]).to_string();
        let m = parse(&s);

        println!("Message: {:?}", m);

        match m.r#type {
            Type::Add => {
                let node = Node("127.0.0.1".to_owned(), m.source_port);
                println!("Added the node to core node list: {:?}", node);
                self.nodes.insert(node);
                println!("Core nodes: {:?}", self.nodes);
            }
            Type::Remove => {
                let node = Node("127.0.0.1".to_owned(), m.source_port);
                println!("Removed the node from core node list: {:?}", node);
                self.nodes.remove(&node);
                println!("Core nodes: {:?}", self.nodes);
            }
        }
    }

    pub fn send_msg(node: &Node, msg: &Message) {
        println!("Sending message: {:?}", msg);
        match TcpStream::connect(format!("{}:{}", node.0, node.1)) {
            Ok(mut stream) => {
                println!("Successfully connected to the node: {:?}", node);

                match stream.write(serde_json::to_string(&msg).unwrap().as_bytes()) {
                    Ok(size) => println!("Sent {} bytes", size),
                    Err(e) => println!("Failed to send message: {:?}", e)
                }
            }
            Err(e) => {
                println!("Failed to connect to the node: {:?}", node);
            }
        }
    }
}
