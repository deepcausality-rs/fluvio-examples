use sbe_bindings::MessageType as SbeMessageType;
use sbe_bindings::{message_header_codec, Encoder, StopDataMsgEncoder, WriteBuf};

use crate::prelude::{SbeEncodeError, StopDataMessage};

impl StopDataMessage {
    pub fn encode(&self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        // Exact buffer size is 14 bytes for the entire message
        let mut buffer = vec![0u8; 16];

        let mut csg = StopDataMsgEncoder::default();

        csg = csg.wrap(
            WriteBuf::new(buffer.as_mut_slice()),
            message_header_codec::ENCODED_LENGTH,
        );

        csg = csg.header(0).parent().expect("Failed to encode header");

        let value = SbeMessageType::from(self.message_type as u16);
        csg.message_type(value);

        let value = self.client_id;
        csg.client_id(value);

        let value = self.exchange_id as u8;
        csg.exchange_id(value);

        let value = self.symbol_id;
        csg.symbol_id(value);

        let value = self.data_type_id as u8;
        csg.data_type_id(value);

        // Limit contains the exact number of bytes required to encode the message
        // Use this value to determine the size of the buffer
        let limit = csg.get_limit();
        Ok((limit, buffer))
    }
}
