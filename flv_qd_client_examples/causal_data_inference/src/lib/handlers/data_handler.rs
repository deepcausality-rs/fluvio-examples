use sbe_messages::prelude::{FirstTradeBar, LastTradeBar, MessageType, SbeTradeBar};
use std::error::Error;
use crate::prelude::CustomModel;

pub fn handle_data_message_inference<'l>(
    value: Vec<u8>,
    _model: &CustomModel<'l>,
) -> Result<(), Box<dyn Error + Send>> {
    // Convert the Vector to a byte slice
    let buffer = value.as_slice();

    // The third byte of the buffer is always the message type.
    let message_type = MessageType::from(buffer[2] as u16);

    match message_type {
        // Handle trade bars
        MessageType::FirstTradeBar => {
            let first_trade_bar = FirstTradeBar::from(buffer);
            println!("{:?}", first_trade_bar);
        }
        MessageType::TradeBar => {
            let trade_bar = SbeTradeBar::decode(buffer).unwrap();
            println!("{:?}", trade_bar);
        }
        MessageType::LastTradeBar => {
            let last_trade_bar = LastTradeBar::from(buffer);
            println!("{:?}", last_trade_bar);
        }
        // Ignore all other message types
        _ => {}
    }

    Ok(())
}
