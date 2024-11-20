fn main() {
    tonic_build::configure()
        .build_server(true) // Enables server code generation
        .build_client(true) // Enables client code generation
        .compile(
            &["proto/cargo_ci.proto"], // Path to the proto file
            &["proto"],               // Include paths
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos: {:?}", e));
}