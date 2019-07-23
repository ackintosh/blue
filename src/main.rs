use std::net::{TcpListener, TcpStream};
use std::io::Read;
use blue::connection_manager::ConnectionManager;

fn main() {
    let mut c = ConnectionManager::new("127.0.0.1".to_owned(), "7878".to_owned());
    c.start();
}
