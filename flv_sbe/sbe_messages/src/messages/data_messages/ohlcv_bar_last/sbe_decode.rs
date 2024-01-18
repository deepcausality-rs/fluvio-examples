use crate::prelude::{LastOHLCVBar, MessageType};
use sbe_bindings::last_data_bar_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{LastDataBarDecoder, MessageHeaderDecoder, ReadBuf, SbeResult};

pub fn decode_last_data_bar_message(buffer: &[u8]) -> SbeResult<LastOHLCVBar> {
    let mut csg = LastDataBarDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u16);
    assert_eq!(message_type, MessageType::LastOHLCVBar);

    let symbol_id = csg.symbol_id();

    let message = LastOHLCVBar::new(symbol_id);

    Ok(message)
}
