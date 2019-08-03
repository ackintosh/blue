use blue::core::{CoreNode, GenesisCoreNode};
use blue::node::Node;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("command line arguments: {:?}", args);

    match args.len() {
        2 => {
            println!("Bootstrapping as a GENESIS Core Node...");
            let port = &args[1];
            let mut core = GenesisCoreNode::new("127.0.0.1".to_owned(), port.clone());
            core.start();
        }
        3 => {
            println!("Bootstrapping as a Core Node...");

            let port = &args[1];
            let genesis_node = Node("127.0.0.1".to_owned(), args[2].clone());

            let mut core = CoreNode::new("127.0.0.1".to_owned(), port.clone(), genesis_node);
            core.start();
        }
        _ => println!("[Usage]\r ./blue port [genesis-node]\r\r port: A port number what node should listen on.\r genesis-node(optional): A port number of GENESIS node.")
    }
}
