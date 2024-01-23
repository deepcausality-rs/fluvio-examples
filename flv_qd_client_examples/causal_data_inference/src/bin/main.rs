use client_utils::{handle_error_utils, handle_utils, print_utils};
use common::prelude::MessageClientConfig;
use qd_client::QDClient;
use std::time::Duration;
use tokio::time::sleep;

const EXAMPLE: &'static str = "Causal Data Inference";

const FN_NAME: &'static str = "causal_data_inference/main";

const CLIENT_ID: u16 = 77;

#[tokio::main]
async fn main() {
    print_utils::print_example_header(EXAMPLE);

    println!("{FN_NAME}: Build Client config for client ID: {CLIENT_ID}",);
    let client_config = MessageClientConfig::new(CLIENT_ID);

    println!("{FN_NAME}: Build QD Client",);
    let client = QDClient::new(CLIENT_ID, client_config.clone())
        .await
        .expect("basic_data_stream/main: Failed to create QD Gateway client");

    // println!("{FN_NAME}: Start the data handler",);
    // let data_topic = client_config.data_channel();
    // tokio::spawn(async move {
    //     if let Err(e) = handle_utils::handle_channel(&data_topic, handle_data_message).await {
    //         eprintln!("[QDClient/new]: Consumer connection error: {}", e);
    //     }
    // });

    println!("{FN_NAME}: Start the error handler",);
    let err_topic = client_config.error_channel();
    tokio::spawn(async move {
        if let Err(e) =
            handle_utils::handle_channel(&err_topic, handle_error_utils::handle_error_message).await
        {
            eprintln!("[QDClient/new]: Consumer connection error: {}", e);
        }
    });

    println!("{FN_NAME}: Wait a moment to let the OHLCV data stream complete...");
    sleep(Duration::from_secs(2)).await;

    println!("{FN_NAME}: Closing client");
    client.close().await.expect("Failed to close client");
}
