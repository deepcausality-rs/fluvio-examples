use client_utils::flv_utils::{create_topic, delete_topic};
use common::prelude::MessageClientConfig;
use fluvio::FluvioAdmin;
use std::error::Error;
use std::process;

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
    let admin = FluvioAdmin::connect()
        .await
        .expect("Failed to connect to Fluvio admin API");

    let client_config = MessageClientConfig::new(42);

    // Setup client connection
    connect(&admin, &client_config)
        .await
        .expect("Failed to setup client connection");

    // Login to the gateway.
    //

    // Logout from the gateway.

    // Close client connection.
    close(&admin, &client_config)
        .await
        .expect("Failed to close client connection");

    Ok(())
}

async fn connect(
    admin: &FluvioAdmin,
    client_config: &MessageClientConfig,
) -> Result<(), Box<dyn Error>> {
    println!("Setup Client Connection!");

    // Create client topics:
    // 1. client_id-control - For receiving control messages from the QD gateway.
    let control_topic = &client_config.control_channel();
    create_topic(&admin, control_topic)
        .await
        .expect("Failed to create control topic");

    // 2. client_id-data - For receiving data messages from the QD gateway.
    let data_topic = &client_config.data_channel();
    create_topic(&admin, data_topic)
        .await
        .expect("Failed to create data topic");

    // 3. client_id-error - For receiving error messages from the QD gateway.
    let err_topic = &client_config.error_channel();
    create_topic(&admin, err_topic).await.expect("Failed to create data topic");

    Ok(())
}

async fn close(
    admin: &FluvioAdmin,
    client_config: &MessageClientConfig,
) -> Result<(), Box<dyn Error>> {
    println!("Close Client Connection!");

    // 1. client_id-control
    let control_topic = &client_config.control_channel();
    delete_topic(&admin, control_topic)
        .await
        .expect("Failed to delete topic");

    // 2. client_id-data
    let data_topic = &client_config.data_channel();
    delete_topic(&admin, data_topic)
        .await
        .expect("Failed to delete topic");

    // 3. client_id-error
    let err_topic = &client_config.error_channel();
    delete_topic(&admin, err_topic)
        .await
        .expect("Failed to delete topic");

    Ok(())
}
