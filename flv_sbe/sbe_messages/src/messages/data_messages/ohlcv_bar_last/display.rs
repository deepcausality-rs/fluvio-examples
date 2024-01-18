use crate::prelude::LastOHLCVBar;
use std::fmt;

impl fmt::Display for LastOHLCVBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "LastOHLCVBar {{ message_type: {:?}, symbol_id: {} }}",
            self.message_type, self.symbol_id
        )
    }
}
