use anyhow::Result;
use clap::{Parser, Subcommand};

mod utils;

use utils::*;

/// A naive httpie http request handler implementation with Rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Customized header to send with query
    #[arg(long)]
    header: Vec<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Http GET request
    Get(command_args::Get),
    /// Http POST request
    Post(command_args::Post),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Get(args) => http_client::get(&args, cli.header).await?,
        Commands::Post(args) => http_client::post(&args, cli.header).await?,
    };

    Ok(())
}
