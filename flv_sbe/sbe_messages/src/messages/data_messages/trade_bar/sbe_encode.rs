use crate::prelude::SbeEncodeError;
use common::prelude::TradeBar;
use rust_decimal::prelude::ToPrimitive;

use sbe_bindings::{
    Encoder, MessageType as SbeMessageType, TradeBarEncoder, WriteBuf, ENCODED_LENGTH,
};

pub fn encode_data_bar_message(bar: TradeBar) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    // precise buffer size is 26 bytes for the entire message.
    let mut buffer = vec![0u8; 26];

    let mut csg = TradeBarEncoder::default();

    csg = csg.wrap(WriteBuf::new(buffer.as_mut_slice()), ENCODED_LENGTH);

    csg = csg.header(0).parent().expect("Failed to encode header");

    let value = SbeMessageType::TradeBar;
    csg.message_type(value);

    let date_time = bar.date_time().timestamp_micros();
    csg.date_time(date_time);

    let price = bar
        .price()
        .to_f32()
        .expect("Failed to convert price to f32");
    csg.price(price);

    let volume = bar
        .volume()
        .to_f32()
        .expect("Failed to convert volume to u64");
    csg.volume(volume);

    let limit = csg.get_limit();
    Ok((limit, buffer))
}
