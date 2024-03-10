use std::time::Duration;

use tokio::time::sleep;

use client_utils::prelude::print_utils;
use common::prelude::{ExchangeID, IggyConfig, IggyUser};
use qd_client::QDClient;
use symdb_client::SymdbClient;

mod handle_data;
mod utils;

const EXAMPLE: &str = "Symbol Master Data Stream";

const FN_NAME: &str = "symbol_master/main";

const CLIENT_ID: u16 = 23;

/// The main function demonstrates streaming trade data for a symbol from the QDGW.
/// It first creates a MessageClientConfig to hold the client configuration.
/// It gets the SymdbClient configuration and creates a SymdbClient to lookup symbol IDs.
/// It creates a QDClient for streaming data from the exchange.
/// It spawns tasks to concurrently handle incoming data and error messages.
/// It looks up the symbol ID for the "ethaed" symbol name using the SymdbClient.
/// Using the symbol ID, it tells the QDClient to start streaming trade data
/// for that symbol from the Kraken exchange
/// It waits briefly to allow some data to stream.
/// Finally it closes the QDClient to end the streaming.
///
/// So in summary, it:
/// - Sets up the client and symbol ID lookup
/// - Starts streaming trade data for a symbol   
/// - Waits briefly to let data flow
/// - Closes the client when done
///
/// The main purpose is demonstrating how to stream  trade data for a
/// specific symbol from the QDGW using the QD client and SYMDB client.
///

#[tokio::main]
async fn main() {
    print_utils::print_example_header(EXAMPLE);

    println!("{FN_NAME}: Build Client config for client ID: {CLIENT_ID}",);
    let user = IggyUser::default();
    let client_config = IggyConfig::from_client_id(user, CLIENT_ID as u32, 50000, false);

    println!("{FN_NAME}: Build SYMDB Client config");
    let symdb_client_config = utils::get_symdb_config();

    println!("{FN_NAME}: Build SYMDB Client",);
    let mut symdb_client = SymdbClient::new(symdb_client_config).await.unwrap();

    println!("{FN_NAME}: Build QD Client",);
    let qd_client = QDClient::new(CLIENT_ID, client_config.clone())
        .await
        .expect("basic_data_stream/main: Failed to create QD Gateway client");

    // println!("{FN_NAME}: Start the data handler",);
    // let data_topic = client_config.data_channel();
    // tokio::spawn(async move {
    //     if let Err(e) = handle_utils::handle_channel(&data_topic, handle_data_message).await {
    //         eprintln!("[QDClient/new]: Consumer connection error: {}", e);
    //     }
    // });
    //
    // println!("{FN_NAME}: Start the error handler",);
    // let err_topic = client_config.error_channel();
    // tokio::spawn(async move {
    //     if let Err(e) =
    //         handle_utils::handle_channel(&err_topic, handle_error_utils::handle_error_message).await
    //     {
    //         eprintln!("[QDClient/new]: Consumer connection error: {}", e);
    //     }
    // });

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
