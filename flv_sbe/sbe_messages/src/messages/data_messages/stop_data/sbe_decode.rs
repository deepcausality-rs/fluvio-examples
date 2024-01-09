use crate::prelude::{MessageType, StopDataMessage};
use common::prelude::{ExchangeID, SymbolID};
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

    let sbe_exchange_id = csg.exchange_id();
    let exchange_id = ExchangeID::from(sbe_exchange_id as i32);

    let sbe_asset = csg.symbol_id();
    let symbol = SymbolID::from(sbe_asset);

    let message = StopDataMessage {
        message_type,
        client_id,
        exchange_id,
        symbol_id: symbol,
    };

    Ok(message)
}
