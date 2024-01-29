use causal_model::prelude::CustomModel;
use deep_causality::prelude::Causable;
use fluvio::Offset;
use futures::stream::StreamExt;
use rust_decimal::prelude::ToPrimitive;
use sbe_messages::prelude::{FirstTradeBar, LastTradeBar, MessageType, SbeTradeBar};
use std::error::Error;
use std::sync::Arc;

const FN_NAME: &'static str = "data_handler/handle_message_inference";

/// Handles messages for running inference.
///
/// The MessageHandler provides methods to subscribe to a data topic,
/// receive messages, run them through an inference model, and handle the
/// results.
///
/// It holds the topic to subscribe to and the inference model to use.
/// The model is wrapped in an Arc to allow shared ownership between
/// instances.
///
pub struct MessageHandler<'l> {
    /// The Fluvio topic to subscribe for receiving data messages.
    channel_topic: String,

    /// The inference model instance. This is wrapped in an Arc to allow
    /// shared ownership between MessageHandler instances.
    model: Arc<CustomModel<'l>>,
}

impl<'l> MessageHandler<'l> {
    /// Create a new MessageHandler instance.
    ///
    /// This initializes a new MessageHandler with the provided channel topic
    /// and inference model.
    ///
    /// # Arguments
    ///
    /// * `channel_topic` - The Fluvio topic to subscribe to for receiving data messages.
    /// * `model` - The inference model to use for processing messages. This should be an
    ///   Arc pointer to allow shared ownership between instances.
    ///
    /// # Returns
    ///
    /// A new MessageHandler instance initialized with the provided args.
    ///
    pub fn new(channel_topic: String, model: Arc<CustomModel<'l>>) -> Self {
        Self {
            channel_topic,
            model,
        }
    }
}

impl<'l> MessageHandler<'l> {
    /// Run inference on received message data.
    ///
    /// This method takes the received message data, converts it to the appropriate format
    /// for the inference model, runs inference, and handles the results.
    ///
    /// It creates a consumer for the specified channel topic to receive messages. It then
    /// creates a stream from that consumer. The stream is iterated, processing each message
    /// by calling handle_message_inference.
    ///
    /// The main logic flow is:
    /// 1. It creates a consumer for the provided channel topic to receive messages.
    /// 2. It creates a stream from the consumer to process the messages asynchronously.
    /// 3. It iterates through the stream, calling handle_message_inference for each message.
    /// 4. handle_message_inference checks the message type, extracts the data, runs inference using the model, and prints any detected events.
    /// 5. If any errors occur during message handling, it returns the error.
    ///
    /// Any errors during message handling are returned to the caller.
    ///
    /// # Arguments
    ///
    /// * `self` - The MessageHandler instance.
    ///
    /// # Returns
    ///
    /// Returns a Result with Ok if inference executed successfully for all messages,
    /// otherwise an Err with the error encountered.
    ///
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

    /// Handles an incoming data message by running inference.
    ///
    /// Takes a data message payload and passes it to the inference function
    /// along with the causal model to run inference.
    ///
    /// # Arguments
    ///
    /// * `value` - The message payload as a byte vector
    ///
    /// # Returns
    ///
    /// A Result with no value if inference succeeds, or an error if it fails.
    ///
    /// # Errors
    ///
    /// Returns any error from the inference function.
    ///
    pub fn handle_message_inference(&self, message: Vec<u8>) -> Result<(), Box<dyn Error + Send>> {
        // The third byte of the buffer is always the message type.
        let message_type = MessageType::from(message[2] as u16);

        match message_type {
            // Handle first trade bar
            MessageType::FirstTradeBar => {
                let first_trade_bar = FirstTradeBar::from(message.as_slice());
                println!("{FN_NAME}: Data stream Starts: {:?}", first_trade_bar);
            }

            // Handle actual trade bar with data
            MessageType::TradeBar => {
                let trade_bar = SbeTradeBar::decode(message.as_slice()).unwrap();

                // Extract the price from the trade bar
                let price = trade_bar.price().to_f64().unwrap();

                // Apply the model to the price for causal inference
                let res = self
                    .model
                    .causaloid()
                    .verify_all_causes(&[price, price], None)
                    .unwrap_or_else(|e| {
                        println!("{FN_NAME}: {}", e);
                        false
                    });

                // Print the result of the inference in case it detected a price breakout
                if res {
                    println!("DeepCausality: Detected Price Breakout!");
                }
            }

            // Handle last trade bar
            MessageType::LastTradeBar => {
                let last_trade_bar = LastTradeBar::from(message.as_slice());
                println!("{FN_NAME}: Data stream stops:{:?}", last_trade_bar);
            }
            // Ignore all other message types
            _ => {}
        }
        Ok(())
    }
}
