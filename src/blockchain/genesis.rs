use crate::blockchain::block::{Block, create_genesis_block};

pub fn genesis_block() -> Block {
    create_genesis_block()
}

pub fn print_genesis_block() {
    let block = genesis_block();
    println!("Genesis Block height: {}", block.height);
    println!("Genesis Block hash: {:x?}", block.hash);
}
