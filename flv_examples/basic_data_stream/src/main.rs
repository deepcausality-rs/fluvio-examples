use crate::handle_data::handle_data_message;
use client_utils::prelude::{handle_error_utils, handle_utils, print_utils};
use common::prelude::{ExchangeID, MessageClientConfig, TimeResolution};
use qd_client::QDClient;
use std::time::Duration;
use tokio::time::sleep;

mod handle_data;

const EXAMPLE: &str = "Basic Data Stream";

const FN_NAME: &str = "basic_data_stream/main";

const CLIENT_ID: u16 = 42;

const ETH_AED: u16 = 284; //  284 = ethaed on Kraken

const OP_USD: u16 = 665; // 665 = opusd on Kraken

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
/// 5) Starts OHLCV data stream
/// - Sends start_ohlcv_data message to gateway
///
/// 6) Waits a bit until the streams have finished
///
/// 7) Closes the client
/// - The client sends a logout message to the gateway,
///  closes the connection, and deletes all client topics.
///
#[tokio::main]
async fn main() {
    print_utils::print_example_header(EXAMPLE);

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

    println!("{FN_NAME}: Wait a moment to let the trade data stream complete...");
    sleep(Duration::from_secs(1)).await;

    // Configure OHLCV data
    let exchange_id = ExchangeID::Kraken;
    let symbol_id = OP_USD;
    let time_resolution = TimeResolution::FiveMin;

    println!("{FN_NAME}: Start streaming 5 MIN OHLCV data for OP/USD with symbol id: {OP_USD}");
    client
        .start_ohlcv_data(exchange_id, symbol_id, time_resolution)
        .await
        .expect("Failed to start OHLCV data");

    println!("{FN_NAME}: Wait a moment to let the OHLCV data stream complete...");
    sleep(Duration::from_secs(3)).await;

    println!("{FN_NAME}: Closing client");
    client.close().await.expect("Failed to close client");
}
