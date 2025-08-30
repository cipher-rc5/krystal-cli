// file: src/main.rs
// description: Command-line interface for the Krystal API client, providing interactive
//             commands for querying pools, positions, and blockchain data with formatted output
// docs_reference: https://docs.rs/clap/latest/clap/

use krystal_cli::run_cli;
use std::process;

#[tokio::main]
async fn main() {
    if let Err(e) = run_cli().await {
        eprintln!("Error: {}", e);
        eprintln!("Suggestion: {}", e.user_message());
        process::exit(1);
    }
}
