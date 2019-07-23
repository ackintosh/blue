use std::net::{TcpListener, TcpStream};
use std::io::Read;
use blue::core::Core;

fn main() {
    let mut core = Core::new("127.0.0.1".to_owned(), "7878".to_owned());
    core.start();
}
