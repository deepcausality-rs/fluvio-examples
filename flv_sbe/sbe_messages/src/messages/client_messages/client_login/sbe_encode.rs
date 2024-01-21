use crate::errors::SbeEncodeError;
use crate::prelude::ClientLoginMessage;
use sbe_bindings::MessageType as SbeMessageType;
use sbe_bindings::{message_header_codec, ClientLoginEncoder, Encoder, WriteBuf};

impl ClientLoginMessage {
    /// Encodes a ClientLoginMessage to a byte buffer.
    ///
    /// # Arguments
    ///
    /// * `self` - ClientLoginMessage to encode
    ///
    /// # Returns
    ///
    /// (usize, `Vec<u8>`) - Tuple of encoded size and byte buffer
    ///
    /// # Errors
    ///
    /// Returns Err if encoding fails
    ///
    /// # Process
    ///
    /// - Create a 12 byte buffer
    /// - Create default ClientLoginEncoder
    /// - Wrap buffer in WriteBuf
    /// - Encode header
    /// - Encode message_type
    /// - Encode client_id
    /// - Return encoded size and buffer
    ///
    pub fn encode(&self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        // precise buffer size is 12 bytes for the entire message.
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
