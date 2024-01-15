use crate::types::data_types::trade_bar::TradeBar;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

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

    pub fn symbol_id(&self) -> u16 {
        self.symbol_id
    }
}
