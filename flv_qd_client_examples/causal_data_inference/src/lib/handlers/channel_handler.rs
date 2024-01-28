use crate::prelude::CustomModel;
use fluvio::Offset;
use futures::stream::StreamExt;
use std::error::Error;
use std::sync::Arc;


pub struct MessageHandler<'l> {
    channel_topic: String,
    pub(crate) model: Arc<CustomModel<'l>>,
}

impl<'l> MessageHandler<'l> {
    pub fn new(
        channel_topic: String,
        model: Arc<CustomModel<'l>>,
    ) -> Self {
        Self {
            channel_topic,
            model,
        }
    }
}

impl<'l> MessageHandler<'l> {
    pub async fn run_inference(&self) -> Result<(), Box<dyn Error + Send>> {
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
            let message = record.get_value().to_vec();

            // Process the record and apply causal model
            match self.handle_data_message_inference(message) {
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
