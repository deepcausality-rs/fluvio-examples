use crate::prelude::{MessageType, StopDataMessage};
use sbe_bindings::{MessageHeaderDecoder, ReadBuf, SbeResult, StopDataMsgDecoder};

use sbe_bindings::stop_data_msg_codec::SBE_TEMPLATE_ID;

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
    let exchange_id = csg.exchange_id();
    let symbol_id = csg.symbol_id();

    let message = StopDataMessage {
        message_type,
        client_id,
        exchange_id,
        symbol_id,
    };

    Ok(message)
}
