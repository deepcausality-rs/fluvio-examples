use crate::prelude::CustomModel;
use fluvio::Offset;
use futures::stream::StreamExt;
use std::error::Error;
use std::sync::{Arc};

type MessageFunction<'l> = fn(value: Vec<u8>, model: Arc<CustomModel<'l>>) -> Result<(), Box<dyn Error + Send>>;


pub struct MessageHandler<'l> {
    channel_topic: String,
    message_handler: MessageFunction<'l>,
    model: Arc<CustomModel<'l>>,
}

impl<'l> MessageHandler<'l> {
    pub fn new(channel_topic: String, message_handler: MessageFunction<'l>, model: Arc<CustomModel<'l>>) -> Self {
        Self { channel_topic, message_handler, model }
    }
}

impl<'l> MessageHandler<'l> {
    pub async fn run_inference(
        &self,
    ) -> Result<(), Box<dyn Error + Send>> {
        // Create consumer for channel topic.
        let consumer = fluvio::consumer(&self.channel_topic, 0)
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
            match (self.message_handler)(buffer.to_vec(), self.model.clone()) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error processing record: {}", e);
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}
