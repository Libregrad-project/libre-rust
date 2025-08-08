mod blockchain;
mod daemon;

fn main() -> std::io::Result<()> {

      let path = "blockchain.dat";

    let mut blockchain = blockchain::chain::Blockchain::new();
    blockchain.append_block(path, &blockchain::genesis::GENESIS_BLOCK)?;
    
    let new_block = blockchain::block::Block {
        height: 1,
        hash: [1u8; 32],
        timestamp: 1680000001,
    };
    blockchain.append_block(path, &new_block)?;
    blockchain.print_chain();

    Ok(())

}