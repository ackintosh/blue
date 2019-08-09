use crate::node::Node;
use crate::connection_manager::send_msg;
use crate::message::{Message, Type};

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
