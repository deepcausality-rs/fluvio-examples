use crate::errors::SbeDecodeError;
use crate::prelude::MessageType;
use chrono::{DateTime, TimeZone, Utc};
use common::prelude::TradeBar;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use sbe_bindings::trade_bar_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{MessageHeaderDecoder, ReadBuf, TradeBarDecoder};

/// Decodes a TradeBar message from a byte buffer.
///
/// # Arguments
///
/// * `buffer` - Byte buffer containing encoded TradeBar message
///
/// # Returns
///
/// Decoded TradeBar on success
///
/// # Errors
///
/// Returns Err if decoding fails
///
/// # Process
///
/// - Create default TradeBarDecoder
/// - Wrap buffer in ReadBuf
/// - Decode header and validate template ID
/// - Decode and validate message_type
/// - Decode symbol_id
/// - Decode date_time as timestamp and create DateTime
/// - Decode price as f32 and convert to Decimal
/// - Decode volume as f32 and convert to Decimal
/// - Create and return TradeBar
///
pub fn decode_trade_bar_message(buffer: &[u8]) -> Result<TradeBar, SbeDecodeError> {
    let mut csg = TradeBarDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u16);
    assert_eq!(message_type, MessageType::TradeBar);

    let symbol_id = csg.symbol_id();

    let sbe_date_time = csg.date_time();
    let date_time: DateTime<Utc> = Utc.timestamp_micros(sbe_date_time).unwrap();

    let sbe_price = csg.price();
    let price = Decimal::from_f32(sbe_price).expect("[FileManager]: Failed to parse open price");

    let sbe_volume = csg.volume();
    let volume = Decimal::from_f32(sbe_volume).expect("[FileManager]: Failed to parse volume");

    let trade_bar = TradeBar::new(symbol_id, date_time, price, volume);

    Ok(trade_bar)
}
