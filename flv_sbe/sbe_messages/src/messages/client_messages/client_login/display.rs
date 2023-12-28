use crate::messages::client_messages::client_login::ClientLoginMessage;
use std::fmt;

impl fmt::Display for ClientLoginMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ClientLoginMessage {{ client_id: {} }}",
            self.client_id(),
        )
    }
}
