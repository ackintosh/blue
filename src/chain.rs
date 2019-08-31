use crate::block::Block;

#[derive(Debug)]
pub struct Chain {
    blocks: Vec<Block>
}

impl Chain {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::create_genesis()],
        }
    }
}
