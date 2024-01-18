use common::prelude::MessageClientConfig;
use qd_client::QDClient;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

mod handle_data_channel;
mod handle_error_channel;

const CLIENT_ID: u16 = 42;

#[tokio::main]
async fn main() {
    let client_config = MessageClientConfig::new(CLIENT_ID);
    let consumer = consumer(client_config.clone()).await;

    // Create a QD Gateway client
    let client = QDClient::new(CLIENT_ID, client_config, consumer)
        .await
        .expect("Failed to create QD Gateway client");

    // Wait a moment ..
    sleep(Duration::from_secs(3)).await;

    // Close the client
    client.close().await.expect("Failed to close client");
}

async fn consumer(client_config: MessageClientConfig) -> Result<(), Box<dyn Error + Send>> {
    let consume_data_handle = tokio::spawn(handle_data_channel::handle_data_channel(
        client_config.clone(),
    ));
    let consume_err_handle = tokio::spawn(handle_error_channel::handle_error_channel(
        client_config.clone(),
    ));

    match tokio::try_join!(consume_data_handle, consume_err_handle) {
        Ok(_) => {}
        Err(e) => {
            println!(
                " Failed to start produce_handle and consume_handle: {:?}",
                e
            );
        }
    }

    Ok(())
}
