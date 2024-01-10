use std::fmt;
use crate::prelude::LastDataBar;

impl fmt::Display for LastDataBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LastDataBar {{ message_type: {:?}, symbol_id: {} }}", self.message_type, self.symbol_id)
    }
}
