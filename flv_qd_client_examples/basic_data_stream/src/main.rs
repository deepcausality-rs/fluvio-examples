use common::prelude::MessageClientConfig;
use qd_client::QDClient;
use std::time::Duration;
use tokio::time::sleep;

mod handle_data_channel;
mod handle_error_channel;

const CLIENT_ID: u16 = 42;

#[tokio::main]
async fn main() {
    let client_config = MessageClientConfig::new(CLIENT_ID);

    //
    let data_handler = handle_data_channel::handle_data_event;
    let error_handler = handle_error_channel::handle_error_event;
    //
    // Create a QD Gateway client
    let client = QDClient::new(CLIENT_ID, client_config, data_handler, error_handler)
        .await
        .expect("Failed to create QD Gateway client");

    // client.start_trade_data().await.expect("Failed to start trade data");

    // Wait a moment ..
    sleep(Duration::from_secs(5)).await;

    // Close the client
    client.close().await.expect("Failed to close client");
}
