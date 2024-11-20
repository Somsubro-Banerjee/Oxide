use tokio::signal;
use tokio::sync::watch;
use tonic::transport::Server;
use crate::grpc::cargo_ci::cargo_ci_server::CargoCiServer;
use crate::grpc::CargoCiService;
use tracing::{info, Level};

pub async fn start_grpc_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = CargoCiService::default();

    // Create a signal channel for graceful shutdown
    let (shutdown_tx, mut shutdown_rx) = watch::channel(());

    // Spawn a task to listen for Ctrl+C
    tokio::spawn(async move {
        if let Err(e) = signal::ctrl_c().await {
            eprintln!("Error listening for shutdown signal: {}", e);
        }
        info!("🔴 Shutdown signal received. Shutting down...");
        let _ = shutdown_tx.send(());
    });

    info!("🔧 gRPC server listening on {} 🟢🟢🟢", addr);

    // Use serve_with_shutdown for graceful shutdown
    Server::builder()
        .add_service(CargoCiServer::new(service))
        .serve_with_shutdown(addr, async {
            let _ = shutdown_rx.changed().await; // Wait for the shutdown signal
        })
        .await?;

    Ok(())
}
