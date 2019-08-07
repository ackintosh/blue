use blue::core::{CoreNode, GenesisCoreNode};
use blue::node::Node;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("command line arguments: {:?}", args);
    let r#type = &args[1];

    if r#type == "core" {
        run_core_node(args);
    } else if r#type == "edge" {
        run_edge_node(args)
    } else {
        println!("Please specify the node type as first argument: `core` or `edge`");
        std::process::exit(1);
    }
}

fn run_core_node(args: Vec<String>) {
    match args.len() {
        3 => {
            println!("Bootstrapping as a GENESIS Core Node...");
            let port = &args[2];
            let mut core = GenesisCoreNode::new("127.0.0.1".to_owned(), port.clone());
            core.start();
        }
        4 => {
            println!("Bootstrapping as a Core Node...");

            let port = &args[2];
            let genesis_node = Node("127.0.0.1".to_owned(), args[3].clone());

            let mut core = CoreNode::new("127.0.0.1".to_owned(), port.clone(), genesis_node);
            core.start();
        }
        _ => {
            println!("[Usage]\r ./blue core {{port}} [{{genesis-node}}]\r\r port: A port number what node should listen on.\r genesis-node(optional): A port number of GENESIS node.");
            std::process::exit(1);
        }
    }
}

fn run_edge_node(args: Vec<String>) {

}
