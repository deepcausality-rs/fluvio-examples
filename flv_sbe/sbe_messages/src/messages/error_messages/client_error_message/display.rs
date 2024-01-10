use crate::prelude::ClientErrorMessage;
use std::fmt;

impl fmt::Display for ClientErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ClientErrorMessage {{ message_type: {:?}, client_id: {}, client_error_type: {:?} }}",
            self.message_type, self.client_id, self.client_error_type
        )
    }
}
