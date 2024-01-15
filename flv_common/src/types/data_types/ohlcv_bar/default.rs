use crate::prelude::OHLCVBar;
use chrono::Utc;
use rust_decimal::Decimal;

impl Default for OHLCVBar {
    fn default() -> Self {
        Self {
            symbol_id: 1,
            date_time: Utc::now(),
            open: Decimal::default(),
            high: Decimal::default(),
            low: Decimal::default(),
            close: Decimal::default(),
            volume: Decimal::default(),
        }
    }
}
