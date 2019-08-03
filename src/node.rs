use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct Node(pub String, pub String);

pub type NodeSet = HashSet<Node>;

