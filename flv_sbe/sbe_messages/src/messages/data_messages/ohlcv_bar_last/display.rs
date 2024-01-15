use crate::prelude::LastOHCLVBar;
use std::fmt;

impl fmt::Display for LastOHCLVBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "LastDataBar {{ message_type: {:?}, symbol_id: {} }}",
            self.message_type, self.symbol_id
        )
    }
}
