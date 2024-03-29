use crate::prelude::{FirstTradeBar, MessageType};
use sbe_bindings::first_trade_bar_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{FirstTradeBarDecoder, MessageHeaderDecoder, ReadBuf, SbeResult};

/// Decodes a FirstTradeBar message from a byte buffer.
///
/// # Arguments
///
/// * `buffer` - Byte buffer containing encoded FirstTradeBar message
///
/// # Returns
///
/// Decoded FirstTradeBar on success
///
/// # Errors
///
/// Returns Err if decoding fails
///
/// # Process
///
/// - Create default FirstTradeBarDecoder
/// - Wrap buffer in ReadBuf
/// - Decode header and validate template ID
/// - Decode and validate message_type
/// - Decode symbol_id
/// - Create and return FirstTradeBar
///
pub fn decode_first_data_bar_message(buffer: &[u8]) -> SbeResult<FirstTradeBar> {
    let mut csg = FirstTradeBarDecoder::default();

    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u16);
    assert_eq!(message_type, MessageType::FirstTradeBar);

    let symbol_id = csg.symbol_id();

    let message = FirstTradeBar::new(symbol_id);

    Ok(message)
}
