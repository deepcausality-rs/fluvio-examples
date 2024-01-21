use sbe_bindings::MessageType as SbeMessageType;
use sbe_bindings::{message_header_codec, Encoder, StartDataMsgEncoder, WriteBuf};

use crate::prelude::{SbeEncodeError, StartDataMessage};

impl StartDataMessage {
    /// Encodes a StartDataMessage to a byte buffer.
    ///
    /// # Arguments
    ///
    /// * `self` - StartDataMessage to encode
    ///
    /// # Returns
    ///
    /// (usize, Vec<u8>) - Tuple containing encoded size and byte buffer
    ///
    /// # Errors
    ///
    /// Returns Err if encoding fails
    ///
    /// # Process
    ///
    /// - Create 17 byte buffer
    /// - Create default StartDataMsgEncoder
    /// - Wrap buffer in WriteBuf
    /// - Encode header
    /// - Encode message_type
    /// - Encode client_id
    /// - Encode exchange_id
    /// - Encode symbol_id
    /// - Encode time_resolution
    /// - Encode data_type_id
    /// - Return encoded size and buffer
    ///
    pub fn encode(&self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        // precise buffer size is 17 bytes for the entire message.
        let mut buffer = vec![0u8; 17];

        let mut csg = StartDataMsgEncoder::default();

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

        let value = self.time_resolution as u8;
        csg.time_resolution(value);

        let value = self.data_type_id as u8;
        csg.data_type_id(value);

        let limit = csg.get_limit();
        Ok((limit, buffer))
    }
}
