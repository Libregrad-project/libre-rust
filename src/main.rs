mod blockchain;
mod daemon;

fn main() {
    // Print Gensis block info
    blockchain::genesis::print_genesis_block();
    daemon::run();
}
