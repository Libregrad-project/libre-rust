mod config;
mod blockchain;
mod daemon;

fn main() -> std::io::Result<()> {
    let path = config::BLOCKS_FILENAME;

    let mut blockchain = match blockchain::chain::Blockchain::load_from_file(path) {
        Ok(chain) => chain,
        Err(_) => blockchain::chain::Blockchain::new(),
    };

    // Append genesis block if chain empty
    if blockchain.chain.is_empty() {
        println!("No existing blockchain found or empty chain, appending genesis block.");
        let genesis = blockchain::genesis::genesis_block();
        blockchain.append_block(path, &genesis)?;
    }

    println!("Blockchain loaded with {} blocks.", blockchain.chain.len());

    blockchain::genesis::print_genesis_block();

    // If chain is empty (file existed but no blocks), append genesis block
    if blockchain.chain.is_empty() {
        let genesis = blockchain::genesis::genesis_block();
        blockchain.append_block(path, &genesis)?;
    }

    let last_block = blockchain.chain.last().unwrap();

    let new_block = blockchain::block::Block {
        height: last_block.height + 1,
        hash: [1u8; 32], // Replace with real hash when ready
        timestamp: 1680000001,
        prev_hash: last_block.hash,
        // transactions: vec![],
    };

    blockchain.append_block(path, &new_block)?;
    blockchain.print_chain();

    println!("Welcome to {} ({})", config::COIN_NAME, config::COIN_SYMBOL);
    println!("Default Ports: P2P - {}, RPC - {}", config::P2P_DEFAULT_PORT, config::RPC_DEFAULT_PORT);

    Ok(())
}
