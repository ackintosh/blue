use crate::transaction::Transaction;
use chrono::{DateTime, Utc, NaiveDateTime};

#[derive(Debug)]
pub struct Block {
    timestamp: i64,
    transaction: Transaction,
    previous_block_hash: String,
}

impl Block {
    pub fn new(timestamp: i64, transaction: Transaction, previous_block_hash: String) -> Self {
        Self {
            timestamp,
            transaction,
            previous_block_hash,
        }
    }

    pub fn create_genesis() -> Self {
        Self {
            timestamp: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(0, 0)
                , Utc
            ).timestamp(),
            transaction: Transaction::new("".to_owned(), "".to_owned(), 1),
            previous_block_hash: "x".to_owned(),
        }
    }
}
