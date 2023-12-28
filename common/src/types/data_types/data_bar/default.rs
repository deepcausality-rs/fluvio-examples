use crate::prelude::SymbolID;
use crate::types::data_types::data_bar::DataBar;
use chrono::Utc;
use rust_decimal::Decimal;

impl Default for DataBar {
    fn default() -> Self {
        Self {
            date_time: Utc::now(),
            symbol: SymbolID::default(),
            open: Decimal::default(),
            high: Decimal::default(),
            low: Decimal::default(),
            close: Decimal::default(),
            volume: Decimal::default(),
        }
    }
}
