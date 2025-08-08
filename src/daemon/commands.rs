use clap::{Subcommand, Parser};

/// Main CLI parser struct (can also live here or in mod.rs)
#[derive(Parser)]
#[command(author, version, about = "CryptoNote Daemon CLI Test")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start,
    Stop,
    Status,
    Restart,
}

pub fn handle_command(cmd: &Commands) {
    match cmd {
        Commands::Start => println!("Daemon started"),
        Commands::Stop => println!("Daemon stopped"),
        Commands::Status => println!("Daemon status: running"),
        Commands::Restart => println!("Daemon restarted"),
    }
}
