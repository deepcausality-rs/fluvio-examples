use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use crate::types::data_types::trade_bar::TradeBar;

impl TradeBar {
    pub fn date_time(&self) -> DateTime<Utc> {
        self.date_time
    }
    pub fn price(&self) -> Decimal {
        self.price
    }
    pub fn volume(&self) -> Decimal {
        self.volume
    }
}
