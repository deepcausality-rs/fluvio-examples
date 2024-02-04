use chrono::{DateTime, TimeZone, Utc};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

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

impl TradeBar {
    pub fn from_pg_row(symbol_id: u16, row: PgRow) -> Self {
        //
        let timestamp = row
            .try_get("timestamp")
            .expect("[TradeBar]: Could not parse timestamp");

        let p = row
            .try_get("price")
            .expect("[TradeBar]: Could not parse price");

        let v = row
            .try_get("volume")
            .expect("[TradeBar]: Could not parse volume");

        let date_time = Utc.from_local_datetime(&timestamp).unwrap();

        let price = Decimal::from_f64(p).expect("[TradeBar]: Could not parse price from f64");

        let volume = Decimal::from_f64(v).expect("[TradeBar]: Could not parse volume from f64");

        Self {
            symbol_id,
            date_time,
            price,
            volume,
        }
    }
}
