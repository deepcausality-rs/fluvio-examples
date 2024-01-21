use sbe_messages::prelude::{
    FirstOHLCVBar, FirstTradeBar, LastOHLCVBar, LastTradeBar, MessageType, SbeOHLCVBar, SbeTradeBar,
};
use std::error::Error;

pub fn handle_data_event(raw_event: Vec<u8>) -> Result<(), Box<dyn Error + Send>> {
    // The third byte of the buffer is always the message type.
    let message_type = MessageType::from(raw_event[2] as u16);
    let buffer = raw_event.as_slice();

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
