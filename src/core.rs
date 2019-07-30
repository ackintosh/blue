use crate::connection_manager::{MessageHandler, send_msg, HealthChecker, HealthCheckHandle};
use crate::message::{Message, Type};
use crate::node::{Node, NodeSet};
use std::sync::{Arc, RwLock};
use timer::Guard;
use std::thread::JoinHandle;

enum State {
    Init,
    Standby,
    ConnectedToNetwork,
    ShuttingDown,
}

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
        self.core.join_network(&self.genesis_node);
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

        let mut mh = MessageHandler::new(
            self.host.clone(),
            self.port.clone(),
            Arc::clone(&self.node_set)
        ).unwrap();

        std::thread::spawn(move || {
            mh.start();
        })
    }

    pub fn start_health_check(&self) -> HealthCheckHandle {
        let hc = HealthChecker::new(Arc::clone(&self.node_set));
        hc.start()
    }

    fn join_network(&mut self, node: &Node) {
        self.state = State::ConnectedToNetwork;
        send_msg(
            node,
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
