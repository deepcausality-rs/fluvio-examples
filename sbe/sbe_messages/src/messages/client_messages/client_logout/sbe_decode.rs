use crate::prelude::{ClientLogoutMessage, MessageType};
use sbe_bindings::client_logout_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{ClientLogoutDecoder, MessageHeaderDecoder, ReadBuf, SbeResult};

pub fn decode_client_logout_message(buffer: &[u8]) -> SbeResult<ClientLogoutMessage> {
    let mut csg = ClientLogoutDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u8);
    assert_eq!(message_type, MessageType::ClientLogout);

    let client_id = csg.client_id();

    let message = ClientLogoutMessage::new(client_id);

    Ok(message)
}
