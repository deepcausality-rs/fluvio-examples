use chrono::{DateTime, TimeZone, Utc};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;
use std::fmt::Debug;

mod default;
mod display;
mod getters;

#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct OHLCVBar {
    symbol_id: u16,
    date_time: DateTime<Utc>,
    open: Decimal,
    high: Decimal,
    low: Decimal,
    close: Decimal,
    volume: Decimal,
}

impl OHLCVBar {
    /// Creates a new OHLCVBar instance with the provided parameters.
    ///
    /// # Parameters
    ///
    /// - `symbol_id` - The symbol ID this bar is for
    /// - `date_time` - The date/time of this bar
    /// - `open` - The opening price
    /// - `high` - The high price
    /// - `low` - The low price
    /// - `close` - The closing price
    /// - `volume` - The volume traded
    ///
    /// # Returns
    ///
    /// A new OHLCVBar instance with the provided fields populated.
    pub fn new(
        symbol_id: u16,
        date_time: DateTime<Utc>,
        open: Decimal,
        high: Decimal,
        low: Decimal,
        close: Decimal,
        volume: Decimal,
    ) -> Self {
        Self {
            symbol_id,
            date_time,
            open,
            high,
            low,
            close,
            volume,
        }
    }
}

impl OHLCVBar {
    /// Creates a new OHLCVBar instance from a postgres database row.
    ///
    /// Parses the timestamp, open, high, low, close, and volume values from the
    /// provided row and uses them to construct a new OHLCVBar instance.
    ///
    /// # Parameters
    ///
    /// - `row` - The database row containing the OHLCV data
    /// - `symbol_id` - The symbol ID this bar is for
    ///
    /// # Returns
    ///
    /// A new OHLCVBar instance populated with the data parsed from the row.
    pub fn from_pg_row(row: &PgRow, symbol_id: u16) -> OHLCVBar {
        //
        let timestamp = row
            .try_get(0)
            .expect("[DataBar]: Could not parse timestamp");

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

        OHLCVBar::new(symbol_id, datetime, open, high, low, close, volume)
    }
}
