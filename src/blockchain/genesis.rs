use super::block::Block;

pub const GENESIS_BLOCK: Block = Block {
    height: 0,
    hash: [0u8; 32],
    timestamp: 1680000000,
};

pub fn print_genesis_block() {
    println!("Genesis Block height: {}", GENESIS_BLOCK.height);
}
