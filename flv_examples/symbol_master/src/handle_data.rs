use sbe_messages::prelude::{
    FirstOHLCVBar, FirstTradeBar, LastOHLCVBar, LastTradeBar, MessageType, SbeOHLCVBar, SbeTradeBar,
};
use std::error::Error;

/// The handle_data_message function handles data messages received from the gateway.
///
/// It takes a value byte vector containing the serialized data message as parameter.
///
/// It gets the message_type from the third byte of the buffer.
///
/// It then matches on the different MessageTypes:
///
/// - MessageType::FirstOHLCVBar:
///   Deserializes the message into a FirstOHLCVBar and prints it.
///
/// - MessageType::OHLCVBar:
///   Deserializes the message into a SbeOHLCVBar and prints it.
///
/// - MessageType::LastOHLCVBar:
///   Deserializes the message into a LastOHLCVBar and prints it.
///
/// - MessageType::FirstTradeBar:
///   Deserializes the message into a FirstTradeBar and prints it.
///
/// - MessageType::TradeBar:
///   Deserializes the message into a SbeTradeBar and prints it.
///
/// - MessageType::LastTradeBar:
///   Deserializes the message into a LastTradeBar and prints it.
///
/// - Other MessageTypes are ignored.
///
/// It returns a Result with no value if successful, otherwise an error.
///
pub fn handle_data_message(value: Vec<u8>) -> Result<(), Box<dyn Error + Send>> {
    let buffer = value.as_slice();

    // The third byte of the buffer is always the message type.
    let message_type = MessageType::from(buffer[2] as u16);

    match message_type {
        // Handle OHLC bars
        MessageType::FirstOHLCVBar => {
            let first_ohlcv_bar = FirstOHLCVBar::from(buffer);
            println!("{:?}", first_ohlcv_bar);
        }
        MessageType::OHLCVBar => {
            let ohlcv_bar = SbeOHLCVBar::decode_data_bar_message(buffer).unwrap();
            println!("{:?}", ohlcv_bar);
        }
        MessageType::LastOHLCVBar => {
            let last_ohlcv_bar = LastOHLCVBar::from(buffer);
            println!("{:?}", last_ohlcv_bar);
        }
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
