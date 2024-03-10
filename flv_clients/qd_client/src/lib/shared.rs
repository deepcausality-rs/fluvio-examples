use crate::QDClient;
use iggy::bytes_serializable::BytesSerializable;
use iggy::client::MessageClient;
use iggy::messages::send_messages::{Message, Partitioning, SendMessages};
use std::error::Error;

impl QDClient {
    /// Sends a message using the given TopicProducer.
    ///
    /// # Arguments
    ///
    /// * `producer` - The TopicProducer to use for sending the message.
    /// * `buffer` - The message content as a byte vector.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` on success, or an `Error` on failure.
    ///
    /// This sends the message using the given TopicProducer. It uses a NULL record key.
    /// After sending, it flushes the producer to ensure the message is sent.
    ///
    pub(crate) async fn send_message(&self, buffer: Vec<u8>) -> Result<(), Box<dyn Error>> {
        // Send the message.

        let producer = self.producer();
        let iggy_config = self.iggy_config();

        // Build message from encoded first bar
        let message =
            Message::from_bytes(buffer.try_into().unwrap()).expect("Failed to create message");

        producer
            .send_messages(&mut SendMessages {
                stream_id: iggy_config.stream_id(),
                topic_id: iggy_config.topic_id(),
                partitioning: Partitioning::partition_id(iggy_config.partition_id()),
                messages: vec![message],
            })
            .await
            .expect("Failed to send message!");

        Ok(())
    }
}
