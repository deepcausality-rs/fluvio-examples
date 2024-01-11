use std::fmt;
use crate::prelude::LastTradeBar;

impl fmt::Display for LastTradeBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LastTradeBar {{ message_type: {}, symbol_id: {} }}",
            self.message_type, self.symbol_id)
    }
}
