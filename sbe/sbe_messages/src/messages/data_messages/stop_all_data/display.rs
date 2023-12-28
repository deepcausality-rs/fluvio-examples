use crate::prelude::StopAllDataMessage;
use std::fmt;

impl fmt::Display for StopAllDataMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "StopAllDataMessage[message_type: {}, client_id: {}, exchange_id: {}]",
            self.message_type, self.client_id, self.exchange_id
        )
    }
}
