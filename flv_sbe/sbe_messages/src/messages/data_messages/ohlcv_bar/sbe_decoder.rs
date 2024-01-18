use crate::errors::SbeDecodeError;
use crate::prelude::MessageType;
use chrono::{DateTime, TimeZone, Utc};
use common::prelude::OHLCVBar;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use sbe_bindings::data_bar_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{DataBarDecoder, MessageHeaderDecoder, ReadBuf};

pub fn decode_data_bar_message(buffer: &[u8]) -> Result<OHLCVBar, SbeDecodeError> {
    let mut csg = DataBarDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u16);
    assert_eq!(message_type, MessageType::OHLCVBar);

    let symbol_id = csg.symbol_id();

    let sbe_date_time = csg.date_time();
    let date_time: DateTime<Utc> = Utc.timestamp_micros(sbe_date_time).unwrap();

    let sbe_open_price = csg.open_price();
    let open =
        Decimal::from_f32(sbe_open_price).expect("[FileManager]: Failed to parse open price");

    let sbe_high_price = csg.high_price();
    let high =
        Decimal::from_f32(sbe_high_price).expect("[FileManager]: Failed to parse high price");

    let sbe_low_price = csg.low_price();
    let low = Decimal::from_f32(sbe_low_price).expect("[FileManager]: Failed to parse low price");

    let sbe_close_price = csg.close_price();
    let close =
        Decimal::from_f32(sbe_close_price).expect("[FileManager]: Failed to parse close price");

    let sbe_volume = csg.volume();
    let volume = Decimal::from_f32(sbe_volume).expect("[FileManager]: Failed to parse volume");

    let data_bar = OHLCVBar::new(symbol_id, date_time, open, high, low, close, volume);

    Ok(data_bar)
}
