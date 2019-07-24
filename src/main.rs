use std::net::{TcpListener, TcpStream};
use std::io::Read;
use blue::core::Core;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("command line arguments: {:?}", args);

    match args.len() {
        2 => {
            println!("Bootstrapping as a GENESIS Core Node...");
            let port = &args[1];
            let mut core = Core::new("127.0.0.1".to_owned(), port.clone());
            core.start();
        }
        3 => {
            println!("Bootstrapping as a Core Node...");

            let port = &args[1];
            let genesis_node = &args[2];

            let mut core = Core::new("127.0.0.1".to_owned(), port.clone());
            core.join_network(&String::from("127.0.0.1"), genesis_node);
            core.start();
        }
        _ => println!("[Usage]\r ./blue port [genesis-node]\r\r port: A port number what node should listen on.\r genesis-node(optional): A port number of GENESIS node.")
    }
}
