use std::error::Error;

use bytes::Bytes;
use iggy::bytes_serializable::BytesSerializable;
use iggy::client::MessageClient;
use iggy::messages::poll_messages::{PollingStrategy, PollMessages};
use iggy::messages::send_messages::{Message, Partitioning, SendMessages};

use common::prelude::IggyConfig;

use crate::QDClient;

const NAME: &str = "QDClient";

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

        println!("{}: Get producer and producer config", NAME);
        let producer = self.producer();
        let producer_config = self.producer_config();

        println!("{}: Build byte encoded message from Vec<u8>", NAME);
        let bytes = Bytes::from(buffer);
        let message = Message::from_bytes(bytes).expect("Failed to create message");

        println!("{}: Send message", NAME);
        producer
            .send_messages(&mut SendMessages {
                stream_id: producer_config.stream_id(),
                topic_id: producer_config.topic_id(),
                partitioning: Partitioning::partition_id(producer_config.partition_id()),
                messages: vec![message],
            })
            .await
            .expect("Failed to send message!");

        println!("{}: OK", NAME);
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
