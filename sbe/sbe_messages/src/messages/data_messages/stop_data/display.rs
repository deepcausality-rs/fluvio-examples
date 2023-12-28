use crate::prelude::StopDataMessage;

use std::fmt;

impl fmt::Display for StopDataMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "StopDataMessage[message_type: {}, client_id: {}, exchange_id: {}, symbol_id: {}]",
            self.message_type, self.client_id, self.exchange_id, self.symbol_id
        )
    }
}
