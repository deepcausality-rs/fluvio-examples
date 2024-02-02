use crate::handle_data::handle_data_message;
use client_utils::{handle_error_utils, handle_utils, print_utils};
use common::prelude::{ExchangeID, MessageClientConfig};
use qd_client::QDClient;
use std::time::Duration;
use symdb_client::SymdbClient;
use tokio::time::sleep;

mod handle_data;
mod utils;

const EXAMPLE: &str = "Symbol Master Data Stream";

const FN_NAME: &str = "symbol_master/main";

const CLIENT_ID: u16 = 23;

#[tokio::main]
async fn main() {
    print_utils::print_example_header(EXAMPLE);

    println!("{FN_NAME}: Build Client config for client ID: {CLIENT_ID}",);
    let client_config = MessageClientConfig::new(CLIENT_ID);

    println!("{FN_NAME}: Build SYMDB Client config");
    let symdb_client_config = utils::get_symdb_config();

    println!("{FN_NAME}: Build SYMDB Client",);
    let mut symdb_client = SymdbClient::new(symdb_client_config).await.unwrap();

    println!("{FN_NAME}: Build QD Client",);
    let qd_client = QDClient::new(CLIENT_ID, client_config.clone())
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

    let symbol = "ethaed".to_string();

    println!("{FN_NAME}: Lookup symbol ID for symbol: {}", symbol);
    let result = symdb_client
        .lookup_symbol_id(ExchangeID::Kraken, symbol.clone())
        .await;

    assert!(result.is_ok());
    let symbol_id = result.unwrap();

    println!(
        "{FN_NAME}: Send start streaming message for {} with symbol id: {}",
        &symbol, &symbol_id
    );
    let exchange_id = ExchangeID::Kraken;
    qd_client
        .start_trade_data(exchange_id, symbol_id)
        .await
        .expect("Failed to send start trade data message");

    println!("{FN_NAME}: Wait a moment to let the trade data stream complete...");
    sleep(Duration::from_secs(1)).await;

    println!("{FN_NAME}: Closing client");
    qd_client.close().await.expect("Failed to close client");
}
