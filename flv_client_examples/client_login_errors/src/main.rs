use client_utils::flv_utils::{create_topic, delete_topic};
use std::error::Error;
use std::process;
use fluvio::FluvioAdmin;

mod handle;
mod handle_login_error;
mod handle_logout_error;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    println!("Client Setup!");

    // Create a new admin client to create new client topics:
    // 1. test-topic-control - For receiving control messages from the QD gateway.
    // 2. test-topic-data - For receiving data messages from the QD gateway.
    // 3. test-topic-error - For receiving error messages from the QD gateway.

    let admin = FluvioAdmin::connect()
        .await
        .expect("Failed to connect to Fluvio admin API");

    // Create a new topic
    let name = "test-topic";

    // Create a new topic
    create_topic(&admin, name).await.expect("Failed to create topic");

    // Login to the gateway.

    // Logout from the gateway.

    // Delete topic before shutting down the client
    delete_topic(&admin, name).await.expect("Failed to delete topic");

    Ok(())
}
