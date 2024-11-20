mod server;
mod grpc;
mod repo;
mod config;
mod utils;

use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 Starting Cargo CI Server...");
    server::start_grpc_server().await
}
