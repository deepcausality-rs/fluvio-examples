use std::error::Error;
use std::process;
use fluvio::FluvioAdmin;
use fluvio::metadata::objects::CommonCreateRequest;
use fluvio::metadata::topic::TopicSpec;
use client_utils::flv_utils::{create_topic, delete_topic};

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

    // Create a new topic
    let name="test-topic";

    // Create a new topic
    create_topic(name).await.expect("Failed to create topic");

    // Delete topic before shutting down the client
    delete_topic(name).await.expect("Failed to delete topic");;

    Ok(())
}
