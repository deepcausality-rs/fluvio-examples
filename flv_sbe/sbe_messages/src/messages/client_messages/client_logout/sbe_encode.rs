use crate::errors::SbeEncodeError;
use crate::prelude::ClientLogoutMessage;
use sbe_bindings::MessageType as SbeMessageType;
use sbe_bindings::{message_header_codec, ClientLogoutEncoder, Encoder, WriteBuf};

impl ClientLogoutMessage {
    pub fn encode(&self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        // precise buffer size is 12 bytes for the entire message.
        let mut buffer = vec![0u8; 11];

        let mut csg = ClientLogoutEncoder::default();

        csg = csg.wrap(
            WriteBuf::new(buffer.as_mut_slice()),
            message_header_codec::ENCODED_LENGTH,
        );

        csg = csg.header(0).parent().expect("Failed to encode header");

        let value = SbeMessageType::from(self.message_type as u16);
        csg.message_type(value);

        let value = self.client_id;
        csg.client_id(value);

        let limit = csg.get_limit();
        Ok((limit, buffer))
    }
}
