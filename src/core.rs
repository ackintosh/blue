use crate::connection_manager::{send_msg, HealthChecker, HealthCheckHandle, notify_node_set};
use crate::message::{Message, Type, parse_node_set_payload};
use crate::node::{Node, NodeSet, State};
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;
use crate::p2p::{JoinNetwork, HandleMessage, P2pNode};
use crate::stringify;
use std::error::Error;


pub struct GenesisCoreNode {
    core: Core,
}

impl GenesisCoreNode {
    pub fn new(host: String, port: String) -> Self {
        Self {
            core: Core::new(host, port),
        }
    }

    pub fn start(&mut self) {
        let message_handler_handle = self.core.start();
        let health_check_handle = self.core.start_health_check();

        message_handler_handle.join();
    }
}

pub struct CoreNode {
    core: Core,
    genesis_node: Node,
}

impl CoreNode {
    pub fn new(host: String, port: String, genesis_node: Node) -> Self {
        Self {
            core: Core::new(host, port),
            genesis_node,
        }
    }

    pub fn start(&mut self) {
        self.core.join_core_network(&self.genesis_node);
        let handle = self.core.start();

        handle.join();
    }
}

struct Core {
    state: State,
    host: String,
    port: String,
    node_set: Arc<RwLock<NodeSet>>,
}

impl P2pNode for Core {
    fn host(&self) -> String {
        self.host.clone()
    }

    fn port_number(&self) -> String {
        self.port.clone()
    }
}

impl JoinNetwork for Core {}

struct CoreMessageHandler {
    host: String,
    port: String,
    node_set: Arc<RwLock<NodeSet>>,
}

impl CoreMessageHandler {
    fn new(host: &String, port: &String, node_set: &Arc<RwLock<NodeSet>>) -> Self {
        Self {
            host: host.clone(),
            port: port.clone(),
            node_set: Arc::clone(node_set)
        }
    }

    fn is_self(&self, node: &Node) -> bool {
        self.host == node.0 && self.port == node.1
    }
}

impl P2pNode for CoreMessageHandler {
    fn host(&self) -> String {
        self.host.clone()
    }

    fn port_number(&self) -> String {
        self.port.clone()
    }
}

impl HandleMessage for CoreMessageHandler {
    fn handle_add(&mut self, message: &Message) -> Result<(), Box<dyn Error>>{
        let node = Node("127.0.0.1".to_owned(), message.source_port.clone());
        println!("Added the node to core node list: {:?}", node);
        self.node_set.write().map_err(stringify)?.insert(node);
        println!("Core nodes: {:?}", self.node_set);

        notify_node_set(&self.host, &self.port, Arc::clone(&self.node_set));
        Ok(())
    }

    fn handle_add_edge(&mut self, message: &Message) -> Result<(), Box<dyn Error>> {
        println!("TODO");
        Ok(())
    }

    fn handle_remove(&mut self, message: &Message) -> Result<(), Box<dyn Error>> {
        let node = Node("127.0.0.1".to_owned(), message.source_port.clone());
        println!("Removed the node from core node list: {:?}", node);
        self.node_set.write().map_err(stringify)?.remove(&node);
        println!("Core nodes: {:?}", self.node_set);

        notify_node_set(&self.host, &self.port, Arc::clone(&self.node_set));
        Ok(())
    }

    fn handle_ping(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        println!("Received a ping message from the port: {}", message.source_port);
        Ok(())
    }

    fn handle_nodes(&mut self, message: &Message) -> Result<(), Box<dyn Error>> {
        println!("Received the NodeSet: {:?}", message);
        let nodes_payload = parse_node_set_payload(&message.payload);
        let mut nodes = self.node_set.write().map_err(stringify).unwrap();
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
        Ok(())
    }
}

impl Core {
    pub fn new(host: String, port: String) -> Self {
        println!("Initializing core node...");

        Self {
            state: State::Init,
            host: host.clone(),
            port: port.clone(),
            node_set: Arc::new(RwLock::new(NodeSet::new())),
        }
    }

    pub fn start(&mut self) -> JoinHandle<()> {
        self.state = State::Standby;

        let mut mh = CoreMessageHandler::new(&self.host, &self.port, &self.node_set);
        std::thread::spawn(move || {
            mh.listen()
        })
    }

    pub fn start_health_check(&self) -> HealthCheckHandle {
        let hc = HealthChecker::new(self.port.clone(), Arc::clone(&self.node_set));
        hc.start()
    }

    fn join_core_network(&mut self, node: &Node) {
        self.join_network(node, crate::message::Type::Add);
        self.state = State::ConnectedToNetwork;
    }

    fn shutdown(&mut self) {
        self.state = State::ShuttingDown;
        println!("Shutdown core node...");
    }

    fn get_state(&self) -> &State {
        &self.state
    }
}
