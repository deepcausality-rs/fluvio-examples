use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tokio_postgres::Row;

mod default;
mod display;
mod getters;

#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct OHLCVBar {
    date_time: DateTime<Utc>,
    open: Decimal,
    high: Decimal,
    low: Decimal,
    close: Decimal,
    volume: Decimal,
}

impl OHLCVBar {
    /// Creates a new DataBar with the provided field values.
    ///
    /// # Parameters
    ///
    /// * `date_time` - DateTime in UTC for the bar timestamp
    /// * `open` - Open price for the bar as a Decimal
    /// * `high` - High price for the bar as a Decimal
    /// * `low` - Low price for the bar as a Decimal
    /// * `close` - Close price for the bar as a Decimal
    /// * `volume` - Volume for the bar as a Decimal
    ///
    /// # Returns
    ///
    /// A new DataBar instance with the provided field values.
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

impl From<&Row> for OHLCVBar {
    /// Creates a new DataBar instance by parsing the provided pg DB Row.
    ///
    /// Extracts the timestamp, open, high, low, close, and volume values
    /// from the row and uses them to construct a new DataBar.
    ///
    /// The timestamp is extracted as a NaiveDateTime and converted to a DateTime in UTC.
    ///
    /// The price fields are extracted as f64 values and converted to Decimal.
    ///
    /// Returns an error if any of the values cannot be parsed from the row.
    fn from(row: &Row) -> Self {
        let timestamp = row.get::<usize, NaiveDateTime>(0);

        let o = row
            .try_get("open")
            .expect("[DataBar]: Could not parse open price");

        let h = row
            .try_get("high")
            .expect("[DataBar]: Could not parse high price");

        let l = row
            .try_get("low")
            .expect("[DataBar]: Could not parse low price");

        let c = row
            .try_get("close")
            .expect("[DataBar]: Could not parse close price");

        let v = row
            .try_get("volume")
            .expect("[DataBar]: Could not parse volume");

        let datetime = Utc.from_local_datetime(&timestamp).unwrap();

        let open = Decimal::from_f64(o).expect("[DataBar]: Could not parse open price from f64");

        let high = Decimal::from_f64(h).expect("[DataBar]: Could not parse high price from f64");

        let low = Decimal::from_f64(l).expect("[DataBar]: Could not parse low price from f64");

        let close = Decimal::from_f64(c).expect("[DataBar]: Could not parse close price from f64");

        let volume = Decimal::from_f64(v).expect("[DataBar]: Could not parse volume from f64");

        Self::new(datetime, open, high, low, close, volume)
    }
}
