use common::prelude::{ExchangeID, MessageClientConfig, TimeResolution};
use qd_client::QDClient;
use std::time::Duration;
use tokio::time::sleep;

mod handle_data_channel;
mod handle_error_channel;

const CLIENT_ID: u16 = 42;

const ETH_AED: u16 = 278; //  278 = ETHAED on Kraken

const OP_USD: u16 = 653; // 250 = OPR/USD on Kraken

#[tokio::main]
async fn main() {
    let client_config = MessageClientConfig::new(CLIENT_ID);

    println!("basic_data_stream/main: Starting client: QDClient");
    let data_handler = handle_data_channel::handle_data_event;
    let error_handler = handle_error_channel::handle_error_event;
    let client = QDClient::new(CLIENT_ID, client_config, data_handler, error_handler)
        .await
        .expect("basic_data_stream/main: Failed to create QD Gateway client");

    println!(
        "basic_data_stream/main: Start streaming trade data for ETH/AED with symbol id: {}",
        ETH_AED
    );
    let exchange_id = ExchangeID::Kraken;
    let symbol_id = ETH_AED;
    client
        .start_trade_data(exchange_id, symbol_id)
        .await
        .expect("Failed to start trade data");

    println!("basic_data_stream/main: Wait a moment to let stream complete...");
    sleep(Duration::from_secs(1)).await;

    println!(
        "basic_data_stream/main: Start streaming 5 MIN OHLCV data for OP/USD with symbol id: {}",
        OP_USD
    );
    let exchange_id = ExchangeID::Kraken;
    let symbol_id = OP_USD;
    let time_resolution = TimeResolution::FiveMin;
    client
        .start_ohlcv_data(exchange_id, symbol_id, time_resolution)
        .await
        .expect("Failed to start OHLCV data");

    println!("basic_data_stream/main: Wait a moment to let stream complete...");
    sleep(Duration::from_secs(5)).await;

    println!("basic_data_stream/main: Closing client");
    client.close().await.expect("Failed to close client");
}
