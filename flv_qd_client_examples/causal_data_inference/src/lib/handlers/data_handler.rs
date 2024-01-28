use crate::prelude::CustomModel;
use deep_causality::prelude::Causable;
use rust_decimal::prelude::ToPrimitive;
use sbe_messages::prelude::{FirstTradeBar, LastTradeBar, MessageType, SbeTradeBar};
use std::error::Error;
use std::sync::{Arc};

/// Handles an incoming data message by running inference.
///
/// Takes a data message payload and passes it to the inference function
/// along with the causal model to run inference.
///
/// # Arguments
///
/// * `value` - The message payload as a byte vector
/// * `model` - The causal model to use for inference
///
/// # Returns
///
/// A Result with no value if inference succeeds, or an error if it fails.
///
/// # Errors
///
/// Returns any error from the inference function.
///
pub fn handle_data_message_inference<'l>(
    value: Vec<u8>,
    model: Arc<CustomModel<'l>>,
) -> Result<(), Box<dyn Error + Send>> {
    let buffer = value.as_slice();

    // The third byte of the buffer is always the message type.
    let message_type = MessageType::from(buffer[2] as u16);

    match message_type {
        // Handle trade bars
        MessageType::FirstTradeBar => {
            let first_trade_bar = FirstTradeBar::from(buffer);
            println!("Data stream Starts: {:?}", first_trade_bar);
        }
        MessageType::TradeBar => {
            let trade_bar = SbeTradeBar::decode(buffer).unwrap();

            // Extract the price from the trade bar
            let price = trade_bar.price().to_f64().unwrap();

            // Apply the model to the price for causal inference
            let res = model
                .causaloid()
                .verify_single_cause(&price)
                .unwrap_or_else(|e| {
                    println!("Error: {}", e);
                    false
                });

            if res {
                println!("DeepCausality: Detected Price Breakout!");
            }

        }
        MessageType::LastTradeBar => {
            let last_trade_bar = LastTradeBar::from(buffer);
            println!("Data stream stops:{:?}", last_trade_bar);
        }
        // Ignore all other message types
        _ => {}
    }

    Ok(())
}
