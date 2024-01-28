use crate::handlers::channel_handler::MessageHandler;
use deep_causality::prelude::Causable;
use rust_decimal::prelude::ToPrimitive;
use sbe_messages::prelude::{FirstTradeBar, LastTradeBar, MessageType, SbeTradeBar};
use std::error::Error;

impl<'l> MessageHandler<'l> {
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
    pub fn handle_data_message_inference(
        &self,
        message: Vec<u8>,
    ) -> Result<(), Box<dyn Error + Send>> {
        // The third byte of the buffer is always the message type.
        let message_type = MessageType::from(message[2] as u16);

        match message_type {
            // Handle first trade bar
            MessageType::FirstTradeBar => {
                let first_trade_bar = FirstTradeBar::from(message.as_slice());
                println!("Data stream Starts: {:?}", first_trade_bar);
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
                    .verify_single_cause(&price)
                    .unwrap_or_else(|e| {
                        println!("Error: {}", e);
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
                println!("Data stream stops:{:?}", last_trade_bar);
            }
            // Ignore all other message types
            _ => {}
        }
        Ok(())
    }
}
