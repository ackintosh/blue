use crate::node::{State, NodeSet, Node};
use std::sync::{Arc, RwLock};
use crate::p2p::{JoinNetwork, ListenMessage};

pub struct EdgeNode {
    state: State,
    host: String,
    port: String,
    core_node: Node,
    node_set: Arc<RwLock<NodeSet>>,
}

impl JoinNetwork for EdgeNode {
    fn port_number(&self) -> String {
        self.port.clone()
    }
}

impl ListenMessage for EdgeNode {}

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
        let message_handler_handle = self.listen_message(&self.host, &self.port, &self.node_set);

        message_handler_handle.join();
    }
}