use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub height: u64,
    pub prev_hash: [u8; 32],
    pub hash: [u8; 32],
    pub timestamp: u64,
}

impl Block {
    
    pub fn compute_hash(&self) -> [u8; 32] {
        #[derive(Serialize)]
        struct BlockData<'a> {
            height: u64,
            prev_hash: &'a [u8; 32],
            timestamp: u64,
            // add other fields here if needed
        }

        let data = BlockData {
            height: self.height,
            prev_hash: &self.prev_hash,
            timestamp: self.timestamp,
        };

        let encoded = bincode::serialize(&data).unwrap();

        let mut hasher = Sha256::new();
        hasher.update(encoded);
        let result = hasher.finalize();

        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&result);
        hash_bytes
    }
}

// Create the genesis block with a real hash
pub fn create_genesis_block() -> Block {
    let mut block = Block {
        height: 0,
        prev_hash: [0u8; 32], // No previous hash for genesis
        hash: [0u8; 32],      // Placeholder, will be computed
        timestamp: 1680000000,
    };

    block.hash = block.compute_hash();
    block
}
