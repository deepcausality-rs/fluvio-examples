use std::error::Error;
use fluvio::Offset;
use futures::stream::StreamExt;
use crate::prelude::CustomModel;

///
type MessageFunction<'l> = fn(value: Vec<u8>, model:  CustomModel<'l>,) -> Result<(), Box<dyn Error + Send>>;


///
pub async fn handle_data_channel_with_inference<'l>(
    channel_topic: &str,
    message_handler: MessageFunction<'l>,
    model: CustomModel<'l>,
)
    -> Result<(), Box<dyn Error + Send>>
{
    // Create consumer for channel topic.
    let consumer = fluvio::consumer(channel_topic, 0)
        .await
        .expect("Failed to create a consumer for data topic");

    // Create stream for consumer.
    let mut stream = consumer
        .stream(Offset::end())
        .await
        .expect("Failed to create a stream");

    // Consume records from the stream and process with the event handlers.
    while let Some(Ok(record)) = stream.next().await {
        let value = record.get_value().to_vec();
        let buffer = value.as_slice();

        // Process the record and apply causal model
        message_handler(buffer.to_vec(), model.clone())?;
    }

    Ok(())
}
