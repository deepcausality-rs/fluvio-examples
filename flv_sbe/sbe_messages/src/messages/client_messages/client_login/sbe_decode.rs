use crate::prelude::{ClientLoginMessage, MessageType};
use sbe_bindings::client_login_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{ClientLoginDecoder, MessageHeaderDecoder, ReadBuf, SbeResult};

/// Decodes a ClientLoginMessage from a byte buffer.
///
/// # Arguments
///
/// * `buffer` - Byte buffer to decode
///
/// # Returns
///
/// Decoded ClientLoginMessage
///
/// # Errors
///
/// Returns Err if decode fails
///
/// # Process
///
/// - Create default ClientLoginDecoder
/// - Wrap buffer in ReadBuf
/// - Decode header and validate template ID
/// - Decode message_type and validate
/// - Decode client_id
/// - Create and return ClientLoginMessage
///
pub fn decode_client_login_message(buffer: &[u8]) -> SbeResult<ClientLoginMessage> {
    let mut csg = ClientLoginDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u16);
    assert_eq!(message_type, MessageType::ClientLogin);

    let client_id = csg.client_id();

    let message = ClientLoginMessage::new(client_id);

    Ok(message)
}
