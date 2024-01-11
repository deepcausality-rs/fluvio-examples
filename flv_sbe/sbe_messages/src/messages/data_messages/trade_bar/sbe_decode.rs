use crate::errors::SbeDecodeError;
use crate::prelude::MessageType;
use chrono::{DateTime, TimeZone, Utc};
use common::prelude::TradeBar;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use sbe_bindings::trade_bar_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{MessageHeaderDecoder, ReadBuf, TradeBarDecoder};

pub fn decode_trade_bar_message(buffer: &[u8]) -> Result<TradeBar, SbeDecodeError> {
    let mut csg = TradeBarDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u16);
    assert_eq!(message_type, MessageType::TradeBar);

    let sbe_date_time = csg.date_time();
    let date_time: DateTime<Utc> = Utc.timestamp_micros(sbe_date_time).unwrap();

    let sbe_price = csg.price();
    let price = Decimal::from_f32(sbe_price).expect("[FileManager]: Failed to parse open price");

    let sbe_volume = csg.volume();
    let volume = Decimal::from_f32(sbe_volume).expect("[FileManager]: Failed to parse volume");

    let trade_bar = TradeBar::new(date_time, price, volume);

    Ok(trade_bar)
}
