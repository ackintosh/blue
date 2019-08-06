extern crate chrono;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use crate::message::{parse, Type, Message, parse_node_set_payload};
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
    pub fn new(host: String, port: String, node_set: Arc<RwLock<NodeSet>>) -> Self {
        println!("Initializing connection manager...");

        Self {
            host: host.clone(),
            port: port.clone(),
            nodes: node_set,
        }
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

                notify_node_set(&self.host, &self.port, Arc::clone(&self.nodes));
            }
            Type::Remove => {
                let node = Node("127.0.0.1".to_owned(), m.source_port);
                println!("Removed the node from core node list: {:?}", node);
                self.nodes.write().map_err(stringify)?.remove(&node);
                println!("Core nodes: {:?}", self.nodes);

                notify_node_set(&self.host, &self.port, Arc::clone(&self.nodes));
            }
            Type::Ping => {
                println!("Received a ping message from the port: {}", m.source_port);
            }
            Type::Nodes => {
                println!("Received the NodeSet: {:?}", m);
                let nodes_payload = parse_node_set_payload(&m.payload);
                let mut nodes = self.nodes.write().map_err(stringify).unwrap();
                nodes.clear();
                for n in nodes_payload.iter() {
                    if self.is_self(&n) {
                        println!("Skipped the node to insert to NodeSet as it's same as myself: {:?}", n);
                        continue;
                    }
                    if !nodes.insert(n.clone()) {
                        println!("Failed to insert the node: {:?}", n);
                    }
                }
                println!("Finished to update NodeSet");
            }
        }

        Ok(())
    }

    fn is_self(&self, node: &Node) -> bool {
        self.host == node.0 && self.port == node.1
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
                match send_msg(node, &Message::new(Type::Ping, port.clone())) {
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
                notify_node_set(&"127.0.0.1".to_owned(), &port, Arc::clone(&nodes));
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

fn notify_node_set(host: &String, port: &String, nodes: Arc<RwLock<NodeSet>>) {
    println!("Notifying the NodeSet to nodes in network");

    let mut handles = vec![];
    let nodes = nodes.read().map_err(stringify).unwrap();

    for n in nodes.iter() {
        let destination = n.clone();
        let source_port = port.clone();
        let mut nodeset_to_send = nodes.clone();
        nodeset_to_send.insert(Node(host.clone(), port.clone()));

        let h = std::thread::spawn(move || {
            match send_msg(&destination, &Message::new_node_sets(source_port, &nodeset_to_send)) {
                Ok(_) => {}
                Err(e) => println!("Failed to send NodeSet: {:?}", e)
            }
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }
}

