use crate::prelude::{LastTradeBar, MessageType};
use sbe_bindings::last_trade_bar_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{LastTradeBarDecoder, MessageHeaderDecoder, ReadBuf, SbeResult};

/// Decodes a LastTradeBar message from a byte buffer.
///
/// # Arguments
///
/// * `buffer` - Byte buffer containing encoded LastTradeBar message
///
/// # Returns
///
/// Decoded LastTradeBar on success
///
/// # Errors
///
/// Returns Err if decoding fails
///
/// # Process
///
/// - Create default LastTradeBarDecoder
/// - Wrap buffer in ReadBuf
/// - Decode header and validate template ID
/// - Decode and validate message_type
/// - Decode symbol_id
/// - Create and return LastTradeBar
///
pub fn decode_last_trade_bar_message(buffer: &[u8]) -> SbeResult<LastTradeBar> {
    let mut csg = LastTradeBarDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u16);
    assert_eq!(message_type, MessageType::LastTradeBar);

    let symbol_id = csg.symbol_id();

    let message = LastTradeBar::new(symbol_id);

    Ok(message)
}
