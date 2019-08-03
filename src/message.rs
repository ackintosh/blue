#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub r#type: Type,
    pub source_port: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    Add,
    Remove,
    Ping,
}

pub fn parse(s: &String) -> Message {
    println!("{}", s);
    serde_json::from_str(s).unwrap()
}
