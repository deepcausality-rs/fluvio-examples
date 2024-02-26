use chrono::{DateTime, TimeZone, Utc};
use klickhouse::{DateTime64, Row};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Row, Serialize, Deserialize)]
pub struct TradeRow {
    date_time: DateTime64<3>,
    price: f64,
    volume: f64,
}

impl TradeRow {
    pub fn date_time(&self) -> DateTime<Utc> {
        Utc.timestamp_millis_opt(self.date_time.1 as i64).unwrap()
    }
    pub fn price(&self) -> Decimal {
        Decimal::from_f64(self.price).unwrap()
    }
    pub fn volume(&self) -> Decimal {
        Decimal::from_f64(self.volume).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Row, Serialize, Deserialize)]
pub struct OHLCVRow {
    datetime: u32,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

impl OHLCVRow {
    pub fn date_time(&self) -> DateTime<Utc> {
        Utc.timestamp_millis_opt(self.datetime as i64).unwrap()
    }
    pub fn open(&self) -> Decimal {
        Decimal::from_f64(self.open).unwrap()
    }
    pub fn high(&self) -> Decimal {
        Decimal::from_f64(self.high).unwrap()
    }
    pub fn low(&self) -> Decimal {
        Decimal::from_f64(self.low).unwrap()
    }
    pub fn close(&self) -> Decimal {
        Decimal::from_f64(self.close).unwrap()
    }
    pub fn volume(&self) -> Decimal {
        Decimal::from_f64(self.volume).unwrap()
    }
}

#[derive(Debug, Eq, Clone, PartialEq, Row, Serialize, Deserialize)]
pub struct SymbolRow {
    symbol_id: u64,
    symbol: String,
}

impl SymbolRow {
    pub fn symbol_id(&self) -> u64 {
        self.symbol_id
    }
    pub fn symbol(&self) -> String {
        self.symbol.to_string()
    }
}
