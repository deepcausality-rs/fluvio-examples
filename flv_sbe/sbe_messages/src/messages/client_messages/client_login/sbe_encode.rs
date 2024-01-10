use crate::errors::SbeEncodeError;
use crate::prelude::ClientLoginMessage;
use sbe_bindings::MessageType as SbeMessageType;
use sbe_bindings::{message_header_codec, ClientLoginEncoder, Encoder, WriteBuf};

impl ClientLoginMessage {
    pub fn encode(&self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        // precise buffer size is 19 bytes for the entire message.
        let mut buffer = vec![0u8; 12];

        let mut csg = ClientLoginEncoder::default();

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
