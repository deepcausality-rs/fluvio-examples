use crate::errors::SbeEncodeError;
use crate::prelude::LastOHLCVBar;
use sbe_bindings::MessageType as SbeMessageType;
use sbe_bindings::{message_header_codec, Encoder, LastDataBarEncoder, WriteBuf};

impl LastOHLCVBar {
    /// Encodes a LastOHLCVBar to a byte buffer.
    ///
    /// # Arguments
    ///
    /// * `self` - LastOHLCVBar to encode
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
    /// - Create 12 byte buffer
    /// - Create default LastDataBarEncoder
    /// - Wrap buffer in WriteBuf
    /// - Encode header
    /// - Encode message_type
    /// - Encode symbol_id
    /// - Return encoded size and buffer
    ///
    pub fn encode(&self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        // precise buffer size is 12 bytes for the entire message.
        let mut buffer = vec![0u8; 12];

        let mut csg = LastDataBarEncoder::default();

        csg = csg.wrap(
            WriteBuf::new(buffer.as_mut_slice()),
            message_header_codec::ENCODED_LENGTH,
        );

        csg = csg.header(0).parent().expect("Failed to encode header");

        let value = SbeMessageType::from(self.message_type as u16);
        csg.message_type(value);

        let value = self.symbol_id;
        csg.symbol_id(value);

        let limit = csg.get_limit();
        Ok((limit, buffer))
    }
}
