use common::prelude::{ExchangeID, MessageClientConfig, TimeResolution};
use qd_client::QDClient;
use std::time::Duration;
use tokio::time::sleep;
use futures::stream::StreamExt;



mod handle_data_channel;
mod handle_error_channel;

const FN_NAME: &'static str = "basic_data_stream/main";

const CLIENT_ID: u16 = 42;

const ETH_AED: u16 = 278; //  278 = ETHAED on Kraken

const OP_USD: u16 = 653; // 250 = OPR/USD on Kraken

#[tokio::main]
async fn main() {
    let client_config = MessageClientConfig::new(CLIENT_ID);

    println!("{FN_NAME}: Starting client: QDClient");
    let data_handler = handle_data_channel::handle_data_event;
    // let error_handler = handle_error_channel::handle_error_event;

    let client = QDClient::new(CLIENT_ID, client_config)
        .await
        .expect("basic_data_stream/main: Failed to create QD Gateway client");

    println!("{FN_NAME}: Send start streaming message for ETH/AED with symbol id: {ETH_AED}",);
    let exchange_id = ExchangeID::Kraken;
    let symbol_id = ETH_AED;
    client
        .start_trade_data(exchange_id, symbol_id)
        .await
        .expect("Failed to send start trade data message");


    let output = client.on_message(data_handler);
    futures::pin_mut!(output);
    while let Some(x) = output.next().await {

        // ^^^^ method cannot be called on `Pin<&mut impl Future<Output = ...>>`
        // due to unsatisfied trait bounds


        println!("{:?}", x);
    }


    println!("{FN_NAME}: Wait a moment to let stream complete...");
    sleep(Duration::from_secs(3)).await;

    println!("{FN_NAME}: Closing client");
    client.close().await.expect("Failed to close client");
}
