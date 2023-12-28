use crate::prelude::{MessageType, StopAllDataMessage};
use common::prelude::ExchangeID;
use sbe_bindings::{MessageHeaderDecoder, ReadBuf, SbeResult, StopAllDataMsgDecoder};

use sbe_bindings::stop_all_data_msg_codec::SBE_TEMPLATE_ID;

pub fn decode_stop_all_data_message(buffer: &[u8]) -> SbeResult<StopAllDataMessage> {
    let mut csg = StopAllDataMsgDecoder::default();

    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u8);
    assert_eq!(message_type, MessageType::StopAllData);

    let client_id = csg.client_id();

    let sbe_exchange_id = csg.exchange_id();
    let exchange_id = ExchangeID::from(sbe_exchange_id as i32);

    let message = StopAllDataMessage {
        message_type,
        client_id,
        exchange_id,
    };

    Ok(message)
}
