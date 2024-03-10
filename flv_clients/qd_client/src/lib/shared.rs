use std::error::Error;

use iggy::client::MessageClient;
use iggy::messages::poll_messages::{PollMessages, PollingStrategy};
use iggy::messages::send_messages::{Message, Partitioning, SendMessages};

use common::prelude::IggyConfig;

use crate::QDClient;

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
    pub(crate) async fn send_message(&self, message: Message) -> Result<(), Box<dyn Error>> {
        let producer = self.producer();
        let producer_config = self.producer_config();

        producer
            .send_messages(&mut SendMessages {
                stream_id: producer_config.stream_id(),
                topic_id: producer_config.topic_id(),
                partitioning: Partitioning::partition_id(producer_config.partition_id()),
                messages: vec![message],
            })
            .await
            .expect("Failed to send message!");

        Ok(())
    }
}

// Preconfigure the poll message command for the consumer client
pub(crate) fn get_poll_command(iggy_config: &IggyConfig) -> PollMessages {
    PollMessages {
        consumer: Default::default(),
        stream_id: iggy_config.stream_id(),
        topic_id: iggy_config.topic_id(),
        partition_id: Option::from(iggy_config.partition_id()),
        strategy: PollingStrategy::last(),
        count: iggy_config.messages_per_batch(),
        auto_commit: iggy_config.auto_commit(),
    }
}
