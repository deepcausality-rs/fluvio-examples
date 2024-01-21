use crate::prelude::{FirstOHLCVBar, MessageType};
use sbe_bindings::first_data_bar_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{FirstDataBarDecoder, MessageHeaderDecoder, ReadBuf, SbeResult};

/// Decodes a FirstOHLCVBar message from a byte buffer.
///
/// # Arguments
///
/// * `buffer` - Byte buffer to decode
///
/// # Returns
///
/// Decoded FirstOHLCVBar
///
/// # Errors
///
/// Returns Err if decode fails
///
/// # Process
///
/// - Create default FirstDataBarDecoder
/// - Wrap buffer in ReadBuf
/// - Decode header and validate template ID
/// - Decode and validate message_type
/// - Decode symbol_id
/// - Create and return FirstOHLCVBar
///
pub fn decode_first_data_bar_message(buffer: &[u8]) -> SbeResult<FirstOHLCVBar> {
    let mut csg = FirstDataBarDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u16);
    assert_eq!(message_type, MessageType::FirstOHLCVBar);

    let symbol_id = csg.symbol_id();

    let message = FirstOHLCVBar::new(symbol_id);

    Ok(message)
}
