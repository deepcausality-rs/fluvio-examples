use crate::prelude::StartDataMessage;
use std::fmt;

impl fmt::Display for StartDataMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "StartDataMessage[message_type: {}, client_id: {}, exchange_id: {}, symbol_id: {}]",
            self.message_type, self.client_id, self.exchange_id, self.symbol_id
        )
    }
}
