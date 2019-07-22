#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    r#type: String,
    source_port: String,
}

pub fn parse(s: &String) -> Message {
    println!("{}", s);
    serde_json::from_str(s).unwrap()
}
