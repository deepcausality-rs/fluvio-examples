use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

mod default;
mod display;
mod getters;

#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeBar {
    symbol_id: u16,
    date_time: DateTime<Utc>,
    price: Decimal,
    volume: Decimal,
}

impl TradeBar {
    /// Creates a new TradeBar with the provided values.
    ///
    /// # Parameters
    ///
    /// * `date_time` - The DateTime of the trade bar
    /// * `price` - The price for the trade bar as a Decimal
    /// * `volume` - The volume for the trade bar as a Decimal
    ///
    /// # Returns
    ///
    /// A new TradeBar instance with the given date_time, price and volume.
    pub fn new(symbol_id: u16, date_time: DateTime<Utc>, price: Decimal, volume: Decimal) -> Self {
        Self {
            symbol_id,
            date_time,
            price,
            volume,
        }
    }
}
