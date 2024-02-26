use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use clickhouse::Row;


mod default;
mod display;
mod getters;

#[derive(Debug, Eq, Clone, PartialEq, Row, Serialize, Deserialize)]
pub struct OHLCVBar {
    symbol_id: u16,
    date_time: DateTime<Utc>,
    open: Decimal,
    high: Decimal,
    low: Decimal,
    close: Decimal,
    volume: Decimal,
}

impl OHLCVBar {
    /// Creates a new OHLCVBar instance with the provided parameters.
    ///
    /// # Parameters
    ///
    /// - `symbol_id` - The symbol ID this bar is for
    /// - `date_time` - The date/time of this bar
    /// - `open` - The opening price
    /// - `high` - The high price
    /// - `low` - The low price
    /// - `close` - The closing price
    /// - `volume` - The volume traded
    ///
    /// # Returns
    ///
    /// A new OHLCVBar instance.
    pub fn new(
        symbol_id: u16,
        date_time: DateTime<Utc>,
        open: Decimal,
        high: Decimal,
        low: Decimal,
        close: Decimal,
        volume: Decimal,
    ) -> Self {
        Self {
            symbol_id,
            date_time,
            open,
            high,
            low,
            close,
            volume,
        }
    }
}
