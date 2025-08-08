use super::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![],
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("Block height: {}, hash: {:?}, timestamp: {}", 
                     block.height, block.hash, block.timestamp);
        }
    }
}