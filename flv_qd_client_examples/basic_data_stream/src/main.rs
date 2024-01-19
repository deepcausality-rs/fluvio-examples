use common::prelude::{ExchangeID, MessageClientConfig};
use qd_client::QDClient;
use std::time::Duration;
use tokio::time::sleep;

mod handle_data_channel;
mod handle_error_channel;

const CLIENT_ID: u16 = 42;

const ETH_AED: u16 = 278;
//  278 = ethaed on Kraken

#[tokio::main]
async fn main() {
    let client_config = MessageClientConfig::new(CLIENT_ID);

    println!("basic_data_stream/main: Starting client: QDClient");
    let data_handler = handle_data_channel::handle_data_event;
    let error_handler = handle_error_channel::handle_error_event;
    let client = QDClient::new(CLIENT_ID, client_config, data_handler, error_handler)
        .await
        .expect("basic_data_stream/main: Failed to create QD Gateway client");

    println!("basic_data_stream/main: Start streaming trade data for ETH/AED");
    let exchange_id = ExchangeID::Kraken;
    let symbol_id = ETH_AED;
    client.start_trade_data(exchange_id, symbol_id)
        .await
        .expect("Failed to start trade data");

    println!("basic_data_stream/main: Wait a moment ...");
    sleep(Duration::from_secs(5)).await;

    println!("basic_data_stream/main: Closing client");
    client.close().await.expect("Failed to close client");
}
