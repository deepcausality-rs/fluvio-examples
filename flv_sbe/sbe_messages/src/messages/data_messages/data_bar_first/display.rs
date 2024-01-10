use std::fmt;
use crate::prelude::FirstDataBar;

impl fmt::Display for FirstDataBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FirstDataBar {{ message_type: {:?}, symbol_id: {} }}", self.message_type, self.symbol_id)
    }
}
