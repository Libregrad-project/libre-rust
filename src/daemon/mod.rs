pub mod commands;

use clap::Parser;
use commands::{Cli, handle_command};

pub fn run() {
    let cli = Cli::parse();
    handle_command(&cli.command);
}
