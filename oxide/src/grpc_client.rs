use crate::grpc_client::cargo_ci::cargo_ci_client::CargoCiClient;
use anyhow::Result;
use std::collections::HashMap;
use tonic::transport::Channel;

pub mod cargo_ci {
    tonic::include_proto!("cargo_ci");
}

pub async fn create_repository(data: &HashMap<&str, String>) -> Result<String> {
    // Connect to the gRPC server
    println!("🔗 Making connection to Cargo CI agent...");
    let mut client = CargoCiClient::connect("http://[::1]:50051").await?;

    // Create a request
    let request = tonic::Request::new(cargo_ci::CreateRepoRequest {
        project_name: data["project_name"].clone(),
        framework: data["framework_stack"].clone(),
        org: data["org"].clone(),
    });
    println!("✅ Connection successful!");
    println!("💻 Building your repository, please wait......");
    // Call the `create_repo` RPC and get the response
    let response = client.create_repo(request).await?.into_inner();
    
    Ok(response.repo_url)
}
