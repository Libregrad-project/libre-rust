use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub height: u64,
   // pub prev_hash: [u8; 32],
    pub hash: [u8; 32],
    pub timestamp: u64,
   // pub transactions: Vec<Transaction>,

}
