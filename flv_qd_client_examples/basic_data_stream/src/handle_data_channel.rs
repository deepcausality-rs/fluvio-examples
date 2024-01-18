use common::prelude::MessageClientConfig;
use fluvio::Offset;
use futures::StreamExt;
use sbe_messages::prelude::MessageType;
use std::error::Error;

pub(crate) async fn handle_data_channel(
    client_config: MessageClientConfig,
) -> Result<(), Box<dyn Error + Send>> {
    let data_topic = client_config.data_channel();

    let consumer = fluvio::consumer(data_topic, 0)
        .await
        .expect("Failed to create a consumer for data topic");

    let mut stream = consumer
        .stream(Offset::end())
        .await
        .expect("Failed to create a stream");

    while let Some(Ok(record)) = stream.next().await {
        let value = record.get_value().to_vec();
        let buffer = value.as_slice();
        let message_type = MessageType::from(buffer[2] as u16);

        match message_type {
            // Handle OHLC bars
            MessageType::FirstOHLCVBar => {}
            MessageType::OHLCVBar => {}
            MessageType::LastOHLCVBar => {}
            // Handle trade bars
            MessageType::FirstTradeBar => {}
            MessageType::TradeBar => {}
            MessageType::LastTradeBar => {}
            // Ignore other message types
            _ => {}
        }
    }

    Ok(())
}
