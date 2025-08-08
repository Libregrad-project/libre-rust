pub const COIN_NAME: &str = "Libre";
pub const COIN_SYMBOL: &str = "LBR";
pub const COIN_DECIMALS: u8 = 8;
pub const COIN_MAX_SUPPLY: u64 = 21_000_000_000_000; // Total supply limit
pub const BLOCK_TIME: u64 = 60; // Block time in seconds
pub const PUBLIC_ADDRESS_PREFIX: u8 = 0x1; 

pub const MIN_TRANSACTION_FEE: u64 = 1000; // Minimum transaction fee in smallest unit
pub const MAX_BLOCK_SIZE: usize = 1_000_000; // Maximum block size

pub const EMISSION_SPEED__FACTOR: u64 = 28; 

pub const BLOCKS_FILENAME: &str = "blocks.dat";
pub const BLOCKINDEXES_FILENAME: &str = "blockindexes.dat";
pub const BLOCKCACHE_FILENAME: &str = "blockcache.dat";
pub const POOLDATA_FILENAME: &str = "poolstate";
pub const P2P_NET_DATA_FILENAME: &str = "p2pstate.dat";

pub const GENESIS_COINBASE_TX_HEX: &str = ""; 

pub const P2P_DEFAULT_PORT: u16 = 18080; 
pub const RPC_DEFAULT_PORT: u16 = 18081;

pub const SEED_NODES: [&str; 3] = [
    "node1.librecoin.org:18080",
    "node2.librecoin.org:18080",
    "node3.librecoin.org:18080",
];