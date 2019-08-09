extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod p2p;
pub mod core;
pub mod edge;
pub mod node;
pub mod connection_manager;
pub mod message;

pub fn stringify(x: impl ToString) -> String {
    x.to_string()
}