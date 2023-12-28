use crate::prelude::SbeEncodeError;
use common::prelude::DataBar;
use rust_decimal::prelude::ToPrimitive;

use sbe_bindings::{
    message_header_codec, DataBarEncoder, Encoder, MessageType as SbeMessageType, WriteBuf,
};

pub fn encode_data_bar_message(bar: DataBar) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    // precise buffer size is xxx bytes for the entire message.
    let mut buffer = vec![0u8; 39];

    let mut csg = DataBarEncoder::default();

    csg = csg.wrap(
        WriteBuf::new(buffer.as_mut_slice()),
        message_header_codec::ENCODED_LENGTH,
    );

    csg = csg.header(0).parent().expect("Failed to encode header");

    let value = SbeMessageType::DataBar;
    csg.message_type(value);

    let date_time = bar.date_time().timestamp_micros();
    csg.date_time(date_time);

    let symbol_id = bar.symbol() as u16;
    csg.symbol_id(symbol_id);

    let open_price = bar
        .open()
        .to_f32()
        .expect("Failed to convert open price to f32");
    csg.open_price(open_price);

    let high_price = bar
        .high()
        .to_f32()
        .expect("Failed to convert high price to f32");
    csg.high_price(high_price);

    let low_price = bar
        .low()
        .to_f32()
        .expect("Failed to convert low price to f32");
    csg.low_price(low_price);

    let close_price = bar
        .close()
        .to_f32()
        .expect("Failed to convert close price to f32");
    csg.close_price(close_price);

    let volume = bar
        .volume()
        .to_f32()
        .expect("Failed to convert volume to u64");
    csg.volume(volume);

    let limit = csg.get_limit();
    Ok((limit, buffer))
}
