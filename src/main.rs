use std::net::{TcpListener, TcpStream};
use std::io::Read;
use blue::core::Core;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("command line arguments: {:?}", args);

    match args.len() {
        1 => {
            println!("Bootstrapping as a GENESIS Core Node...");
            let mut core = Core::new("127.0.0.1".to_owned(), "7878".to_owned());
            core.start();
        }
        3 => {
            println!("Bootstrapping as a Core Node...");
            let mut core = Core::new("127.0.0.1".to_owned(), "7879".to_owned());

            let core_host = &args[1];
            let core_port = &args[2];
            core.join_network(core_host, core_port);
            core.start();
        }
        _ => panic!()
    }
}
