use crate::utils as client_utils;
use common::prelude::{ExchangeID, MessageClientConfig, SymbolID};
use fluvio::Offset;
use futures::StreamExt;
use sbe_messages::prelude::{
    ClientLoginMessage, ClientLogoutMessage, DataType, MessageType, SbeOHLCVBar, StartDataMessage,
};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

const CLIENT_ID: u16 = 100;

pub async fn handle() -> Result<(), Box<dyn Error>> {
    let produce_handle = tokio::spawn(produce());
    let consume_handle = tokio::spawn(consume());

    match tokio::try_join!(produce_handle, consume_handle) {
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

async fn produce() -> Result<(), Box<dyn Error + Send>> {
    let service_topic = "qdgw-control";
    let producer = client_utils::get_producer(service_topic).await;

    let message = ClientLoginMessage::new(CLIENT_ID);

    let enc = message.encode();
    let (_, buffer) = enc.unwrap();
    client_utils::send_message(&producer, buffer)
        .await
        .expect("Failed to send message!");

    sleep(Duration::from_millis(20)).await;

    let exchange_id = ExchangeID::BinanceSpot;
    let symbol_id = SymbolID::BTCUSD as u16;
    let data_type = DataType::TradeData;
    let message = StartDataMessage::new(CLIENT_ID, exchange_id, symbol_id, data_type);

    let enc = message.encode();
    let (_, buffer) = enc.unwrap();
    client_utils::send_message(&producer, buffer)
        .await
        .expect("Failed to send message!");

    sleep(Duration::from_millis(1000)).await;

    let message = ClientLogoutMessage::new(CLIENT_ID);

    let enc = message.encode();
    assert!(enc.is_ok());

    let (_, buffer) = enc.unwrap();
    client_utils::send_message(&producer, buffer)
        .await
        .expect("Failed to send message!");

    Ok(())
}

async fn consume() -> Result<(), Box<dyn Error + Send>> {
    let client_config = MessageClientConfig::new(CLIENT_ID);

    let topic = client_config.data_channel();

    let consumer = client_utils::get_consumer(&topic).await;

    let mut stream = consumer
        .stream(Offset::end())
        .await
        .expect("Failed to create a stream");

    while let Some(Ok(record)) = stream.next().await {
        let value = record.get_value().to_vec();
        let buffer = value.as_slice();

        let message_type = MessageType::from(buffer[2] as u16);
        if message_type == MessageType::LastDataBar {
            println!("[zmq manager]: Last data bar.");
            println!("[zmq manager]: EXIT NOW");
            break;
        }

        let bar = SbeOHLCVBar::decode_data_bar_message(buffer).expect("Failed to decode data bar");
        println!("[zmq manager]: Message type: {}", message_type);
        println!("[zmq manager]: Message: \n {}", bar);
    }

    Ok(())
}
