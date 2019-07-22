use std::net::{TcpListener, TcpStream};
use std::io::Read;

struct Node(String, String);

struct ConnectionManager {
    host: String,
    port: String,
    nodes: Vec<Node>,
}

impl ConnectionManager {
    fn new(host: String, port: String) -> Self {
        println!("Initializing connection manager...");
        Self {
            host,
            port,
            nodes: vec![Node(host, port)]
        }
    }

    fn start(&self) {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap();

        for stream in listener.incoming() {
            Self::handle_connection(stream.unwrap());
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 512];

        stream.read(&buffer).unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    }
}
