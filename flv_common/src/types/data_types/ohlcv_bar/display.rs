use crate::prelude::OHLCVBar;
use std::fmt;
use std::fmt::Display;

impl Display for OHLCVBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DataTime: {},\n Open {},\n High {},\n Low {},\n Close {},\n Volume {}",
            self.date_time, self.open, self.high, self.low, self.close, self.volume
        )
    }
}
