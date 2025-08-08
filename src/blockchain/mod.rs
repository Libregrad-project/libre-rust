pub mod block;
pub mod chain;

pub fn print_block_info() {
    println!("Block info:");
    block::print_block();
    chain::print_chain();
}
