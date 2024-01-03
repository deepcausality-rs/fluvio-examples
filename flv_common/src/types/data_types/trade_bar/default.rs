use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use crate::types::data_types::trade_bar::TradeBar;

impl Default for TradeBar {

    fn default() -> Self {
        Self {
            date_time: Utc::now(),
            price: Decimal::zero(),
            volume: Decimal::zero()
        }
    }
}
