use crate::node::{State, NodeSet, Node};
use std::sync::{Arc, RwLock};
use crate::p2p::{JoinNetwork, HandleMessage, P2pNode};
use crate::message::Message;
use std::error::Error;

pub struct EdgeNode {
    state: State,
    host: String,
    port: String,
    core_node: Node,
    node_set: Arc<RwLock<NodeSet>>,
}

impl P2pNode for EdgeNode {
    fn host(&self) -> String {
        self.host.clone()
    }

    fn port_number(&self) -> String {
        self.port.clone()
    }
}

impl JoinNetwork for EdgeNode {}

impl EdgeNode {
    pub fn new(host: String, port: String, core_node_port: String) -> Self {
        Self {
            state: State::Init,
            host,
            port,
            core_node: Node("127.0.0.1".to_owned(), core_node_port),
            node_set: Arc::new(RwLock::new(NodeSet::new())),
        }
    }

    pub fn start(self) {
        self.join_network(&self.core_node, crate::message::Type::AddEdge).unwrap();

        let mut mh = EdgeMessageHandler::new(&self.host, &self.port, &self.core_node, &self.node_set);
        let handler = std::thread::spawn(move || {
            mh.listen();
        });
        handler.join().unwrap();
    }
}

struct EdgeMessageHandler {
    host: String,
    port: String,
    core_node: Node,
    node_set: Arc<RwLock<NodeSet>>,
}

impl EdgeMessageHandler {
    fn new(host: &String, port: &String, core_node: &Node, node_set: &Arc<RwLock<NodeSet>>) -> Self {
        Self {
            host: host.clone(),
            port: port.clone(),
            core_node: core_node.clone(),
            node_set: Arc::clone(node_set)
        }
    }
}

impl P2pNode for EdgeMessageHandler {
    fn host(&self) -> String {
        self.host.clone()
    }

    fn port_number(&self) -> String {
        self.port.clone()
    }
}

impl HandleMessage for EdgeMessageHandler {
    fn handle_add(&mut self, message: &Message) -> Result<(), Box<dyn Error>>{
        println!("[!] Received tye Type::Add message but edge node can't process it: {:?}", message);
        Ok(())
    }

    fn handle_add_edge(&mut self, message: &Message) -> Result<(), Box<dyn Error>> {
        println!("[!] Received tye Type::AddEdge message but edge node can't process it: {:?}", message);
        Ok(())
    }

    fn handle_remove(&mut self, message: &Message) -> Result<(), Box<dyn Error>> {
        println!("[!] Received tye Type::Remove message but edge node can't process it: {:?}", message);
        Ok(())
    }
    fn handle_ping(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        println!("Received a ping message from the port: {}", message.source_port);
        Ok(())
    }
    fn handle_nodes(&mut self, _message: &Message) -> Result<(), Box<dyn Error>> {
        println!("TODO");
        Ok(())
    }
}
