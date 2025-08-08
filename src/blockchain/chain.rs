use super::block::Block;
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write, Read};

#[derive(Serialize, Deserialize, Debug, Default)]
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

    // Save the entire chain to a file (overwrite)
    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let encoded = bincode::serialize(&self).unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        file.write_all(&encoded)?;
        Ok(())
    }

    // Load entire chain from a file
    pub fn load_from_file(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let blockchain: Blockchain = bincode::deserialize(&buffer).unwrap();
        Ok(blockchain)
    }
}
