use causal_model::prelude::CustomModel;
use fluvio::Offset;
use futures::stream::StreamExt;
use std::error::Error;
use std::sync::Arc;

const FN_NAME: &'static str = "message_handler/run_inference";

pub struct MessageHandler<'l> {
    channel_topic: String,
    pub model: Arc<CustomModel<'l>>,
}

impl<'l> MessageHandler<'l> {
    pub fn new(channel_topic: String, model: Arc<CustomModel<'l>>) -> Self {
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
            match self.handle_message_inference(message) {
                Ok(_) => {}
                Err(e) => {
                    println!("{FN_NAME}: Error processing record: {}", e);
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}
