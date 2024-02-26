use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use clickhouse_derive::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, Clone, PartialEq, Row, Serialize, Deserialize)]
pub struct TradeRow {
    date_time: DateTime<Utc>,
    price: Decimal,
    volume: Decimal,
}

impl TradeRow {
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

#[derive(Debug, Eq, Clone, PartialEq, Row, Serialize, Deserialize)]
pub struct OHLCVRow {
    date_time: DateTime<Utc>,
    open: Decimal,
    high: Decimal,
    low: Decimal,
    close: Decimal,
    volume: Decimal,
}

impl OHLCVRow {
    pub fn date_time(&self) -> DateTime<Utc> {
        self.date_time
    }
    pub fn open(&self) -> Decimal {
        self.open
    }
    pub fn high(&self) -> Decimal {
        self.high
    }
    pub fn low(&self) -> Decimal {
        self.low
    }
    pub fn close(&self) -> Decimal {
        self.close
    }
    pub fn volume(&self) -> Decimal {
        self.volume
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
