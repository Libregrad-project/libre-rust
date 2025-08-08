mod blockchain;
mod daemon;

fn main() -> std::io::Result<()> {

      let path = "blockchain.dat";

      let mut blockchain = blockchain::chain::Blockchain::new();
      let genesis = blockchain::genesis::genesis_block();
      blockchain::genesis::print_genesis_block();
      // blockchain.append_block(path, &blockchain::genesis::GENESIS_BLOCK)?;

      let new_block = blockchain::block::Block {
          height: 1,
          hash: [1u8; 32],
          timestamp: 1680000001,
          prev_hash: genesis.hash,
      };
      blockchain.append_block(path, &new_block)?;
      blockchain.print_chain();

      println!("Genesis block hash bytes: {:?}", genesis.hash);

      Ok(())

}