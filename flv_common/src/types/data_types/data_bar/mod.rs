use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

mod default;
mod display;
mod getters;

#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataBar {
    date_time: DateTime<Utc>,
    open: Decimal,
    high: Decimal,
    low: Decimal,
    close: Decimal,
    volume: Decimal,
}

impl DataBar {
    pub fn new(
        date_time: DateTime<Utc>,
        open: Decimal,
        high: Decimal,
        low: Decimal,
        close: Decimal,
        volume: Decimal,
    ) -> Self {
        Self {
            date_time,
            open,
            high,
            low,
            close,
            volume,
        }
    }
}
