use crate::node::{Node, NodeSet};
use crate::connection_manager::{send_msg, MessageHandler};
use crate::message::{Message, Type};
use std::thread::JoinHandle;
use std::sync::{Arc, RwLock};

pub trait JoinNetwork {
    fn port_number(&self) -> String;

    fn join_network(&self, node: &Node) {
        send_msg(
            node,
            &Message::new(
                Type::Add,
                self.port_number()
            )
        );
    }
}

pub trait ListenMessage {
    fn listen_message(&self, host: &String, port: &String, node_set: &Arc<RwLock<NodeSet>>) -> JoinHandle<()> {
        let mut mh = MessageHandler::new(
            host.clone(),
            port.clone(),
            Arc::clone(node_set)
        );

        std::thread::spawn(move || {
            mh.start();
        })
    }
}
