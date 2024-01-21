use crate::prelude::SbeEncodeError;
use common::prelude::OHLCVBar;
use rust_decimal::prelude::ToPrimitive;

use sbe_bindings::{
    message_header_codec, DataBarEncoder, Encoder, MessageType as SbeMessageType, WriteBuf,
};

/// Encodes an OHLCVBar to a byte buffer.
///
/// # Arguments
///
/// * `bar` - OHLCVBar to encode
///
/// # Returns
///
/// (usize, `Vec<u8>`) - Tuple containing encoded size and byte buffer
///
/// # Errors
///
/// Returns Err if encoding fails
///
/// # Process
///
/// - Create 40 byte buffer
/// - Create default DataBarEncoder
/// - Wrap buffer in WriteBuf
/// - Encode header
/// - Encode message_type
/// - Encode symbol_id
/// - Encode date_time
/// - Encode and convert open_price to f32
/// - Encode and convert high_price to f32
/// - Encode and convert low_price to f32
/// - Encode and convert close_price to f32
/// - Encode and convert volume to f32
/// - Return encoded size and buffer
///
pub fn encode_data_bar_message(bar: OHLCVBar) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    // precise buffer size is 40 bytes for the entire message.
    let mut buffer = vec![0u8; 40];

    let mut csg = DataBarEncoder::default();

    csg = csg.wrap(
        WriteBuf::new(buffer.as_mut_slice()),
        message_header_codec::ENCODED_LENGTH,
    );

    csg = csg.header(0).parent().expect("Failed to encode header");

    let value = SbeMessageType::DataBar;
    csg.message_type(value);

    let symbol_id = bar.symbol_id();
    csg.symbol_id(symbol_id);

    let date_time = bar.date_time().timestamp_micros();
    csg.date_time(date_time);

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
