use crate::handle_data::handle_data_message;
use client_utils::prelude::{handle_error_utils, handle_utils};
use common::prelude::{ExchangeID, MessageClientConfig};
use qd_client::QDClient;
use std::time::Duration;
use tokio::time::sleep;

mod handle_data;

const FN_NAME: &'static str = "basic_data_stream/main";

const CLIENT_ID: u16 = 42;

const ETH_AED: u16 = 278; //  278 = ETHAED on Kraken

/// Basic Example of how to use the QD Client to get trade data for a specific symbol.
///  1) Construct QD Client
/// - Creates MessageClientConfig
/// - Creates QDClient instance with config
///
/// 2) Start data handler
/// - Gets data topic from config
/// - Spawns tokio task to handle data channel
///   using handle_channel and handle_data_message
///
/// 3) Start error handler
/// - Gets error topic from config
/// - Spawns tokio task to handle error channel
///   using handle_channel and handle_error_message
///
/// 4) Start trade data stream
/// - Sends start_trade_data message to gateway
///
/// 5) Close connection
/// - Closes QDClient
///
#[tokio::main]
async fn main() {
    println!("{FN_NAME}: Build Client config for client ID: {CLIENT_ID}",);
    let client_config = MessageClientConfig::new(CLIENT_ID);

    println!("{FN_NAME}: Build QD Client",);
    let client = QDClient::new(CLIENT_ID, client_config.clone())
        .await
        .expect("basic_data_stream/main: Failed to create QD Gateway client");

    println!("{FN_NAME}: Start the data handler",);
    let data_topic = client_config.data_channel();
    tokio::spawn(async move {
        if let Err(e) = handle_utils::handle_channel(&data_topic, handle_data_message).await {
            eprintln!("[QDClient/new]: Consumer connection error: {}", e);
        }
    });

    println!("{FN_NAME}: Start the error handler",);
    let err_topic = client_config.error_channel();
    tokio::spawn(async move {
        if let Err(e) =
            handle_utils::handle_channel(&err_topic, handle_error_utils::handle_error_message).await
        {
            eprintln!("[QDClient/new]: Consumer connection error: {}", e);
        }
    });

    println!("{FN_NAME}: Send start streaming message for ETH/AED with symbol id: {ETH_AED}",);
    let exchange_id = ExchangeID::Kraken;
    let symbol_id = ETH_AED;
    client
        .start_trade_data(exchange_id, symbol_id)
        .await
        .expect("Failed to send start trade data message");

    println!("basic_data_stream/main: Wait a moment to let stream complete...");
    sleep(Duration::from_secs(3)).await;

    println!("{FN_NAME}: Closing client");
    client.close().await.expect("Failed to close client");
}
