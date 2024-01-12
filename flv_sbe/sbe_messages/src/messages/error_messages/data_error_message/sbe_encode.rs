use crate::errors::SbeEncodeError;
use crate::prelude::DataErrorMessage;
use sbe_bindings::MessageType as SbeMessageType;
use sbe_bindings::{message_header_codec, DataErrorEncoder, Encoder, WriteBuf};

impl DataErrorMessage {
    pub fn encode(&self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        let mut buffer = vec![0u8; 13];

        let mut csg = DataErrorEncoder::default();

        csg = csg.wrap(
            WriteBuf::new(buffer.as_mut_slice()),
            message_header_codec::ENCODED_LENGTH,
        );

        csg = csg.header(0).parent().expect("Failed to encode header");

        let value = SbeMessageType::from(self.message_type as u16);
        csg.message_type(value);

        let value = self.client_id;
        csg.client_id(value);

        let value = self.data_error_type as u8;
        csg.data_error_type(value);

        let limit = csg.get_limit();
        Ok((limit, buffer))
    }
}
