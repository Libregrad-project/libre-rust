pub struct Block {
    pub height: u32,
    pub hash: [u8; 32],
    pub timestamp: u64,
}

pub const GENESIS_BLOCK: Block = Block {
    height: 0,
    hash: [0u8; 32],
    timestamp: 1680000000,
};

pub fn print_block() {
    println!("Block height: {}", GENESIS_BLOCK.height);
}
