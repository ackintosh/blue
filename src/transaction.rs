#[derive(Debug)]
pub struct Transaction {
    sender: String,
    recipient: String,
    value: i64,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, value: i64) -> Self {
        Self {
            sender,
            recipient,
            value,
        }
    }
}
