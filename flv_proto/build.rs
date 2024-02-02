fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(&["proto/symdb.proto"], &["proto"])
        .expect("Failed to compile proto specification");

    Ok(())
}
