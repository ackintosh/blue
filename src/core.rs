use crate::connection_manager::{MessageHandler};
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
    host: String,
    port: String,
    node_set: Arc<RwLock<NodeSet>>,
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

    pub fn start_as_genesis(&mut self) {
        self.state = State::Standby;

        let mut mh = MessageHandler::new(
            self.host.clone(),
            self.port.clone(),
            Arc::clone(&self.node_set)
        ).unwrap();

        let h = std::thread::spawn(move || {
            mh.start();
        });

        h.join().unwrap();
    }

    pub fn start(&mut self, genesis_host: &String, genesis_port: &String) {
        self.join_network(genesis_host, genesis_port);

        println!("Not implemented yet");
    }

    fn join_network(&mut self, host: &String, port: &String) {
        self.state = State::ConnectedToNetwork;
        MessageHandler::send_msg(
            &Node(host.clone(), port.clone()),
            &Message{
                r#type: Type::Add,
                source_port: self.port.clone(),
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
