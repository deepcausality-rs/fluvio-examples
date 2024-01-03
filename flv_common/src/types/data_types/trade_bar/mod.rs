use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

mod default;
mod getters;
mod display;

#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeBar {
    date_time: DateTime<Utc>,
    price: Decimal,
    volume: Decimal,
}

impl TradeBar {
    pub fn new(date_time: DateTime<Utc>, price: Decimal, volume: Decimal) -> Self {
        Self { date_time, price, volume }
    }
}
