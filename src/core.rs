use crate::connection_manager::{ConnectionManager};
use crate::message::{Message, Type};
use crate::node::{Node, NodeSet};
use std::sync::{Arc, RwLock};

enum State {
    Init,
    Standby,
    ConnectedToNetwork,
    ShuttingDown,
}

pub struct Core {
    state: State,
    cm: ConnectionManager,
}

impl Core {
    pub fn new(host: String, port: String) -> Self {
        println!("Initializing core node...");
        let node_set = Arc::new(RwLock::new(NodeSet::new()));

        Self {
            state: State::Init,
            cm: ConnectionManager::new(
                host.clone(),
                port.clone(),
                Arc::clone(&node_set)
            ).unwrap(),
        }
    }

    pub fn start(&mut self) {
        self.state = State::Standby;
        self.cm.start();
    }

    pub fn join_network(&mut self, host: &String, port: &String) {
        self.state = State::ConnectedToNetwork;
        ConnectionManager::send_msg(
            &Node(host.clone(), port.clone()),
            &Message{
                r#type: Type::Add,
                source_port: self.cm.port.clone(),
            }
        );
    }

    fn shutdown(&mut self) {
        self.state = State::ShuttingDown;
        println!("Shutdown core node...");
    }

    fn get_state(&self) -> &State {
        &self.state
    }
}
