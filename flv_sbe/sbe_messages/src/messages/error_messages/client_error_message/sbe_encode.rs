use crate::errors::SbeEncodeError;
use crate::prelude::ClientErrorMessage;
use sbe_bindings::MessageType as SbeMessageType;
use sbe_bindings::{message_header_codec, ClientErrorEncoder, Encoder, WriteBuf};

/// Encodes a ClientErrorMessage to a byte buffer.
///
/// # Arguments
///
/// * `self` - ClientErrorMessage to encode
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
/// - Create 13 byte buffer
/// - Create default ClientErrorEncoder
/// - Wrap buffer in WriteBuf
/// - Encode header
/// - Encode message_type
/// - Encode client_id
/// - Encode client_error_type
/// - Return encoded size and buffer
///
impl ClientErrorMessage {
    pub fn encode(&self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        let mut buffer = vec![0u8; 13];

        let mut csg = ClientErrorEncoder::default();

        csg = csg.wrap(
            WriteBuf::new(buffer.as_mut_slice()),
            message_header_codec::ENCODED_LENGTH,
        );

        csg = csg.header(0).parent().expect("Failed to encode header");

        let value = SbeMessageType::from(self.message_type as u16);
        csg.message_type(value);

        let value = self.client_id;
        csg.client_id(value);

        let value = self.client_error_type as u8;
        csg.client_error_type(value);

        let limit = csg.get_limit();
        Ok((limit, buffer))
    }
}
