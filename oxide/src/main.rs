mod onboarding;
mod storage;
mod grpc_client;
mod utils;

use clap::{Arg, Command};
use tracing::{info, Level};
use tracing_subscriber;
 

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    
    // Default action if no subcommand is passed
    let matches = Command::new("Oxide")
        .version("1.0.0")
        .about("Interactive CI/CD Framework Manager")
        .subcommand(
            Command::new("start")
                .about("Interactive onboarding for a new CI/CD setup"),
        )
        .subcommand(
            Command::new("oxide-chains")
                .about("View persisted CI/CD configurations"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("start", _)) => onboarding::start_onboarding().await,
        Some(("oxide-chains", _)) => storage::display_persisted_data(),
        _ => println!("Use --help to see available commands."),
    }
}

// A function to prompt the user for the project they want to use
fn prompt_user_for_project_choice() -> usize {
    use std::io::{self, Write};

    let mut input = String::new();
    print!("Enter the number of the project to use: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let input: usize = input.trim().parse().unwrap_or(0);
    input - 1 // Convert to zero-based index
}
