use crate::node::Node;
use crate::connection_manager::{send_msg};
use crate::message::{Message, Type, parse};
use std::net::{TcpStream, TcpListener};
use std::error::Error;
use std::io::Read;

pub trait P2pNode {
    fn host(&self) -> String;
    fn port_number(&self) -> String;
}

pub trait JoinNetwork: P2pNode {
    fn join_network(&self, node: &Node, message_type: Type) {
        send_msg(
            node,
            &Message::new(
                message_type,
                self.port_number()
            )
        );
    }
}

pub trait HandleMessage: P2pNode {
    fn listen(&mut self) -> () {
        println!("Listening on {}:{}", self.host(), self.port_number());
        let listener = TcpListener::bind(format!("{}:{}", self.host(), self.port_number())).unwrap();

        for stream in listener.incoming() {
            self.handle(&stream.unwrap());
        }
    }

    fn handle(&mut self, mut stream: &TcpStream) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0u8; 512];

        let size = stream.read(&mut buffer).unwrap();
        let s = String::from_utf8_lossy(&buffer[..size]).to_string();
        let m = parse(&s);

        println!("Message: {:?}", m);

        match m.r#type {
            Type::Add => self.handle_add(&m),
            Type::AddEdge => self.handle_add_edge(&m),
            Type::Remove => self.handle_remove(&m),
            Type::Ping => self.handle_ping(&m),
            Type::Nodes => self.handle_nodes(&m)
        }
    }

    fn handle_add(&mut self, message: &Message) -> Result<(), Box<dyn Error>>;
    fn handle_add_edge(&mut self, message: &Message) -> Result<(), Box<dyn Error>>;
    fn handle_remove(&mut self, message: &Message) -> Result<(), Box<dyn Error>>;
    fn handle_ping(&self, message: &Message) -> Result<(), Box<dyn Error>>;
    fn handle_nodes(&mut self, message: &Message) -> Result<(), Box<dyn Error>>;
}
