// build.rs in oxide project
use tonic_build;

fn main() {
    tonic_build::configure()
        .compile(
            &["proto/cargo_ci.proto"], // Path to your proto file
            &["proto"],                // Include paths
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos: {:?}", e));
}
