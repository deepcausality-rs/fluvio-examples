use sbe_bindings::{message_header_codec, Encoder, StopAllDataMsgEncoder, WriteBuf};
use sbe_bindings::{ExchangeID as SbeExchangeID, MessageType as SbeMessageType};

use crate::prelude::{SbeEncodeError, StopAllDataMessage};

impl StopAllDataMessage {
    pub fn encode(&self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        // precise buffer size is 10 bytes for the entire message.
        let mut buffer = vec![0u8; 12];

        let mut csg = StopAllDataMsgEncoder::default();

        csg = csg.wrap(
            WriteBuf::new(buffer.as_mut_slice()),
            message_header_codec::ENCODED_LENGTH,
        );

        csg = csg.header(0).parent().expect("Failed to encode header");

        let value = SbeMessageType::from(self.message_type as u8);
        csg.message_type(value);

        let value = self.client_id;
        csg.client_id(value);

        let value = SbeExchangeID::from(self.exchange_id as u8);
        csg.exchange_id(value);

        let limit = csg.get_limit();
        Ok((limit, buffer))
    }
}
