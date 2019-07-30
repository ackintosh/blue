extern crate chrono;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use crate::message::{parse, Type, Message};
use crate::node::{Node, NodeSet};
use std::sync::{Arc, RwLock};
use crate::stringify;
use std::error::Error;
use timer::{Timer, Guard};

pub struct MessageHandler {
    host: String,
    port: String,
    nodes: Arc<RwLock<NodeSet>>,
}

impl MessageHandler {
    pub fn new(host: String, port: String, node_set: Arc<RwLock<NodeSet>>) -> Result<Self, Box<dyn Error>> {
        println!("Initializing connection manager...");

        let c = Self {
            host: host.clone(),
            port: port.clone(),
            nodes: node_set,
        };
        c.nodes.write().map_err(stringify)?.insert(Node(host.clone(), port.clone()));
        Ok(c)
    }

    pub fn start(&mut self) {
        println!("Listening on {}:{}", self.host, self.port);
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap();

        for stream in listener.incoming() {
            self.handle_connection(stream.unwrap());
        }
    }


    fn handle_connection(&mut self, mut stream: TcpStream) -> Result<(), Box<dyn Error>>{
        let mut buffer = [0; 512];

        let size = stream.read(&mut buffer).unwrap();
        let s = String::from_utf8_lossy(&buffer[..size]).to_string();
        let m = parse(&s);

        println!("Message: {:?}", m);

        match m.r#type {
            Type::Add => {
                let node = Node("127.0.0.1".to_owned(), m.source_port);
                println!("Added the node to core node list: {:?}", node);
                self.nodes.write().map_err(stringify)?.insert(node);
                println!("Core nodes: {:?}", self.nodes);
            }
            Type::Remove => {
                let node = Node("127.0.0.1".to_owned(), m.source_port);
                println!("Removed the node from core node list: {:?}", node);
                self.nodes.write().map_err(stringify)?.remove(&node);
                println!("Core nodes: {:?}", self.nodes);
            }
            Type::Ping => {
                println!("Received a ping message from the port: {}", m.source_port);
            }
        }

        Ok(())
    }
}

pub struct HealthChecker {
    port: String,
    nodes: Arc<RwLock<NodeSet>>,
}

pub struct HealthCheckHandle {
    timer: Timer,
    guard: Guard,
}

impl HealthChecker {
    pub fn new(port: String, nodes: Arc<RwLock<NodeSet>>) -> Self {
        Self { port, nodes }
    }

    pub fn start(&self) -> HealthCheckHandle {
        let timer = Timer::new();
        let nodes = self.nodes.clone();
        let port = self.port.clone();
        let guard = timer.schedule_repeating(time::Duration::seconds(3), move || {
            let mut nodes_to_remove: NodeSet = NodeSet::new();

            for node in nodes.read().unwrap().iter() {
                println!("Pinging to {:?}", node);
                match send_msg(node, &Message { r#type: Type::Ping, source_port: port.clone() }) {
                    Ok(_) => println!("OK, {:?} is working fine!", node),
                    Err(e) => {
                        println!("Oops, {:?} seems not healthy.", e);
                        nodes_to_remove.insert(node.clone());
                    }
                }
            }

            if nodes_to_remove.is_empty() {
                println!("Nothing to remove from NodeSet.");
            } else {
                for node in nodes_to_remove {
                    nodes.write().unwrap().remove(&node);
                    println!("Removed the {:?} from NodeSet.", node);
                }
            }
        });

        HealthCheckHandle { timer, guard }
    }
}

pub fn send_msg(node: &Node, msg: &Message) -> Result<(), String>{
    println!("Sending message: {:?}", msg);
    match TcpStream::connect(format!("{}:{}", node.0, node.1)) {
        Ok(mut stream) => {
            println!("Successfully connected to the node: {:?}", node);

            match stream.write(serde_json::to_string(&msg).unwrap().as_bytes()) {
                Ok(size) => {
                    println!("Sent {} bytes", size);
                    return Ok(())
                }
                Err(e) => {
                    return Err(format!("Failed to send message: {:?}", e));
                }
            }
        }
        Err(e) => {
            return Err(format!("Failed to connect to the node: {:?}, error: {:?}", node, e));
        }
    }
}
