use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom, BufReader};
use crate::blockchain::block::Block;
use bincode;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { chain: Vec::new() }
    }

    // Append a block to the file and add it to memory chain
    pub fn append_block(&mut self, path: &str, block: &Block) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        let encoded = bincode::serialize(block).unwrap();
        let length = encoded.len() as u64;

        // Write length prefix
        file.write_all(&length.to_le_bytes())?;
        // Write block bytes
        file.write_all(&encoded)?;

        self.chain.push(block.clone());

        Ok(())
    }

    // Load all blocks from the append-only file into memory
    pub fn load_from_file(path: &str) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut chain = Vec::new();

        loop {
            let mut len_buf = [0u8; 8];
            // Try read length prefix (8 bytes)
            if let Err(_) = reader.read_exact(&mut len_buf) {
                break; // EOF reached or error, stop reading
            }
            let length = u64::from_le_bytes(len_buf);

            let mut block_buf = vec![0u8; length as usize];
            reader.read_exact(&mut block_buf)?;

            let block: Block = bincode::deserialize(&block_buf).unwrap();
            chain.push(block);
        }

        Ok(Blockchain { chain })
    }

    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("Block height: {}, hash: {:?}, timestamp: {}", 
                     block.height, block.hash, block.timestamp);
        }
    }
}
