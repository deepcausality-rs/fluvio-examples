use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::fmt::Debug;

use crate::prelude::SymbolID;
use serde::{Deserialize, Serialize};

mod default;
mod display;
mod getters;

#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataBar {
    date_time: DateTime<Utc>,
    symbol: SymbolID,
    open: Decimal,
    high: Decimal,
    low: Decimal,
    close: Decimal,
    volume: Decimal,
}

impl DataBar {
    pub fn new(
        date_time: DateTime<Utc>,
        symbol: SymbolID,
        open: Decimal,
        high: Decimal,
        low: Decimal,
        close: Decimal,
        volume: Decimal,
    ) -> Self {
        Self {
            date_time,
            symbol,
            open,
            high,
            low,
            close,
            volume,
        }
    }
}
