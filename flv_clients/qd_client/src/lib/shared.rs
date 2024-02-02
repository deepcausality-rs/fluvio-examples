use crate::QDClient;
use fluvio::RecordKey;
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
        self.producer
            .send(RecordKey::NULL, buffer)
            .await
            .expect("[QDClient/send_message]: Failed to send Done!");

        // Flush the producer to ensure the message is sent.
        self.producer
            .flush()
            .await
            .expect("[QDClient/send_message]: Failed to flush to message bus.");

        Ok(())
    }
}
