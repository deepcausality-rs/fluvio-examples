use crate::prelude::CustomModel;
use fluvio::Offset;
use futures::stream::StreamExt;
use std::error::Error;

/// MessageFunction is a function type that handles incoming messages for a channel.
///
/// It takes the message payload as a `Vec<u8>` and a model instance, and returns a `Result`
/// indicating whether the message was processed successfully.
///
/// This allows customizing the message handling logic for different channels by passing
/// different MessageFunction implementations. The model instance allows the handler to maintain
/// state and perform inference.
///
/// # Arguments
///
/// * `value` - The message payload as bytes
/// * `model` - The model instance
///
/// # Returns
///
/// `Result` indicating whether message processing succeeded
///
type MessageFunction<'l> =
    fn(value: Vec<u8>, model: CustomModel<'l>) -> Result<(), Box<dyn Error + Send>>;

// Move this into a process hanlder that can be wrapped in an Arc/Mutex

//
pub async fn handle_data_channel_with_inference<'l>(
    channel_topic: &str,
    message_handler: MessageFunction<'l>,
    model: CustomModel<'l>,
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

    // Consume records from the stream and process with the event handlers.
    while let Some(Ok(record)) = stream.next().await {
        let value = record.get_value().to_vec();
        let buffer = value.as_slice();

        // Process the record and apply causal model
        message_handler(buffer.to_vec(), model.clone())?;
    }

    Ok(())
}
