use crate::prelude::{ClientErrorMessage, ClientErrorType, MessageType};
use sbe_bindings::client_error_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{ClientErrorDecoder, MessageHeaderDecoder, ReadBuf, SbeResult};

pub fn decode_client_error_message(buffer: &[u8]) -> SbeResult<ClientErrorMessage> {
    let mut csg = ClientErrorDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u16);
    assert_eq!(message_type, MessageType::ClientError);

    let client_id = csg.client_id();
    let client_error_type_raw = csg
        .client_error_type()
        .expect("Failed to decode client error type");
    let client_error_type = ClientErrorType::from(client_error_type_raw);

    let message = ClientErrorMessage::new(client_id, client_error_type);

    Ok(message)
}
