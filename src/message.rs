use crate::node::NodeSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub r#type: Type,
    pub source_port: String,
    pub payload: String,
}

impl Message {
    pub fn new(r#type: Type, source_port: String) -> Self {
        Self {
            r#type,
            source_port,
            payload: "".to_owned(),
        }
    }

    pub fn new_node_sets(source_port: String, nodes: &NodeSet) -> Self {
        Self {
            r#type: Type::Nodes,
            source_port,
            payload: serde_json::to_string(nodes).unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    Add,
    AddEdge,
    Remove,
    Ping,
    Nodes,
}

pub fn parse(s: &String) -> Message {
    println!("{}", s);
    serde_json::from_str(s).unwrap()
}

pub fn parse_node_set_payload(payload: &String) -> NodeSet {
    serde_json::from_str(payload).unwrap()
}
