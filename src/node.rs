use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Node(pub String, pub String);

pub type NodeSet = HashSet<Node>;

