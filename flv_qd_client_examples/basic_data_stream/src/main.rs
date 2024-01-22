use common::prelude::{ExchangeID, MessageClientConfig};
use futures::stream::StreamExt;
use qd_client::QDClient;
use sbe_messages::prelude::MessageType;
use std::error::Error;
use fluvio::{Offset, PartitionConsumer};

mod handle_data;
mod handle_error;

const FN_NAME: &'static str = "basic_data_stream/main";

const CLIENT_ID: u16 = 42;

const ETH_AED: u16 = 278; //  278 = ETHAED on Kraken

// Example of how to use the QD Client to get trade data for a specific symbol.
//
// Main function:
// - Creates QDClient instance
// - Sends message to start streaming trade data
// - Gets message stream from client
// - Calls handle_message on messages
// - Closes client
//
// handle_message function:
// - Gets message type
// - Calls specific handler functions based on message type
// - Returns Result
//
// Helper modules handle different message types
// - handle_data handles data messages
// - handle_error handles error messages

#[tokio::main]
async fn main() {
    println!("{FN_NAME}: Build Client config for client ID: {CLIENT_ID}",);
    let client_config = MessageClientConfig::new(CLIENT_ID);

    println!("{FN_NAME}: Build QD Client",);
    let client = QDClient::new(CLIENT_ID, client_config)
        .await
        .expect("basic_data_stream/main: Failed to create QD Gateway client");


    println!("{FN_NAME}: Start the data handler",);
    let data_topic = client_config.data_channel();
    tokio::spawn(async move {
        if let Err(e) = handle_channel(&data_topic).await {
            eprintln!("[QDClient/new]: Consumer connection error: {}", e);
        }
    });


    println!("{FN_NAME}: Start the error handler",);
    let err_topic = client_config.error_channel();
    tokio::spawn(async move {
        if let Err(e) = handle_channel(&err_topic).await {
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


    println!("{FN_NAME}: Closing client");
    client.close().await.expect("Failed to close client");
}



async fn handle_channel(
    channel_topic: &str,
) -> Result<(), Box<dyn Error + Send>> {
    // Create consumer for channel topic.
    let consumer = fluvio::consumer(channel_topic, 0)
        .await
        .expect("Failed to create a consumer for data topic");

    // Create stream for consumer.
    let mut stream = consumer
        .stream(Offset::end())
        .await
        .expect("Failed to create a stream");

    // Consume records from the stream and process with the event handler.
    while let Some(Ok(record)) = stream.next().await {
        let value = record.get_value().to_vec();
        let buffer = value.as_slice();

        handle_message(buffer.to_vec())?;
    }

    Ok(())
}

const HANDLE_NAME: &'static str = "basic_data_stream/main";


async  fn handle_message(msg: Vec<u8>) -> Result<(), Box<dyn Error + Send>> {

    // The third byte of the buffer is always the message type.
    let message_type = MessageType::from(msg[2] as u16);
    println!("{HANDLE_NAME}: Received message type: {message_type:?}");

    match message_type {
        // Handle client errors
        MessageType::ClientError => {
            println!("{HANDLE_NAME}: Received client error message");
            handle_error::handle_client_error(msg).expect("Failed to handle client error");
        }
        // Handle data errors
        MessageType::DataError => {
            println!("{HANDLE_NAME}: Received data error message");
            handle_error::handle_data_error(msg).expect("Failed to handle data error");
        }
        // Handle everything else as data message
        _ => {
            println!("{HANDLE_NAME}: Received data message");
            handle_data::handle_data_message(msg).expect("Failed to process data message");
        }
    }

    Ok(())
}
