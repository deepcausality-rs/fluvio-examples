use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

mod default;
mod display;
mod getters;

#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeBar {
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
    pub fn new(date_time: DateTime<Utc>, price: Decimal, volume: Decimal) -> Self {
        Self {
            date_time,
            price,
            volume,
        }
    }
}

impl From<&Row> for TradeBar {
    /// Creates a new TradeBar from a database Row.
    ///
    /// Parses the timestamp, price, and volume values from the row
    /// and uses them to construct a TradeBar.
    ///
    /// The timestamp is extracted as a NaiveDateTime and converted to a
    /// DateTime in UTC.
    ///
    /// The price and volume fields are extracted as f64 values and
    /// converted to Decimal.
    ///
    /// Returns an error if any of the values cannot be parsed from the row.
    fn from(row: &Row) -> Self {
        let timestamp = row.get::<usize, NaiveDateTime>(0);

        let p = row.try_get(1).expect("[TradeBar]: Could not parse price");

        let v = row.try_get(2).expect("[TradeBar]: Could not parse volume");

        let date_time = Utc.from_local_datetime(&timestamp).unwrap();

        let price = Decimal::from_f64(p).expect("[TradeBar]: Could not parse price from f64");

        let volume = Decimal::from_f64(v).expect("[TradeBar]: Could not parse volume from f64");

        Self {
            date_time,
            price,
            volume,
        }
    }
}
