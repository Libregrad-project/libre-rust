// mod blockchain;

// fn main() {
//     blockchain::genesis::print_genesis_block();
// }

mod blockchain;
mod daemon;

fn main() {
    daemon::run();
}
