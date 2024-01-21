use crate::prelude::{DataType, MessageType, StopDataMessage};
use common::prelude::ExchangeID;
use sbe_bindings::{MessageHeaderDecoder, ReadBuf, SbeResult, StopDataMsgDecoder};

use sbe_bindings::stop_data_msg_codec::SBE_TEMPLATE_ID;

/// Decodes a StopDataMessage from a byte buffer.
///
/// # Arguments
///
/// * `buffer` - Byte buffer to decode
///
/// # Returns
///
/// Decoded StopDataMessage
///
/// # Errors
///
/// Returns Err if decode fails
///
/// # Process
///
/// - Create default StopDataMsgDecoder
/// - Wrap buffer in ReadBuf
/// - Decode header and validate template ID
/// - Decode and validate message_type
/// - Decode client_id
/// - Decode and create exchange_id
/// - Decode symbol_id
/// - Decode and create data_type_id
/// - Create and return StopDataMessage
///
pub fn decode_stop_data_message(buffer: &[u8]) -> SbeResult<StopDataMessage> {
    let mut csg = StopDataMsgDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u16);
    assert_eq!(message_type, MessageType::StopData);

    let client_id = csg.client_id();
    let exchange_id = ExchangeID::from(csg.exchange_id());
    let symbol_id = csg.symbol_id();
    let data_type_id = DataType::from(csg.data_type_id());

    let message = StopDataMessage {
        message_type,
        client_id,
        exchange_id,
        symbol_id,
        data_type_id,
    };

    Ok(message)
}
