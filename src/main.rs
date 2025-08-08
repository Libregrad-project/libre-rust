mod blockchain;
mod daemon;

// fn main() {
//     // Print Gensis block info
//     blockchain::genesis::print_genesis_block();


    
//     daemon::run();
// }

fn main() {
    let mut blockchain = blockchain::chain::Blockchain::new();
    blockchain.add_block(blockchain::genesis::GENESIS_BLOCK);
  //  blockchain.print_chain();

    let new_block = blockchain::block::Block {
        height: 1,
        hash: [1u8; 32],
        timestamp: 1680000001,
    };
    blockchain.add_block(new_block);
    blockchain.print_chain();

    // Save the blockchain to a file
    if let Err(e) = blockchain.save_to_file("blockchain.dat") {
        eprintln!("Error saving blockchain: {}", e);
    }

    let loaded_blockchain = match blockchain::chain::Blockchain::load_from_file("blockchain.dat") {
        Ok(chain) => chain,
        Err(e) => {
            eprintln!("Error loading blockchain: {}", e);
            return;
        }
    };

  

}