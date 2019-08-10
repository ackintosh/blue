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

impl HandleMessage for EdgeNode {
    fn handle_add(&mut self, message: &Message) -> Result<(), Box<dyn Error>>{
        println!("TODO");
        Ok(())
    }
    fn handle_remove(&mut self, message: &Message) -> Result<(), Box<dyn Error>> {
        println!("TODO");
        Ok(())
    }
    fn handle_ping(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        println!("TODO");
        Ok(())
    }
    fn handle_nodes(&mut self, message: &Message) -> Result<(), Box<dyn Error>> {
        println!("TODO");
        Ok(())
    }
}

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
        self.join_network(&self.core_node);
    }
}
