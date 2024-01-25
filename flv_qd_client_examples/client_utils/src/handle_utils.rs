use fluvio::Offset;
use futures::stream::StreamExt;
use std::error::Error;

/// Handles consuming messages from a Fluvio topic and processing them.
///
/// # Parameters
///
/// * `channel_topic` - The Fluvio topic to consume from.
/// * `message_handler` - The handlers function to process each message.
///   Takes the message buffer as parameter and returns a Result.
///
/// # Returns
///
/// Returns a Result with no value if successful, otherwise an error.
///
/// This does the following:
///
/// - Creates a Fluvio consumer for the topic.
/// - Gets a stream for the consumer.
/// - Loops through records from the stream.
///   - Calls the handlers function, passing the message buffer.
///
/// Any errors from the handlers are propagated up.
///
pub async fn handle_channel(
    channel_topic: &str,
    message_handler: fn(buffer: Vec<u8>) -> Result<(), Box<dyn Error + Send>>,
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

        message_handler(buffer.to_vec())?;
    }

    Ok(())
}
