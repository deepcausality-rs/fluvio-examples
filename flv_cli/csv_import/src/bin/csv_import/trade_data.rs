use clickhouse_derive::Row;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct TradeData {
    timestamp: u64,
    price: f64,
    volume: f64,
}

impl TradeData {
    pub fn new(timestamp: u64, price: f64, volume: f64) -> Self {
        Self {
            timestamp,
            price,
            volume,
        }
    }
}

impl Display for TradeData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "TradeData {{ timestamp: {}, price: {}, volume: {} }}",
            self.timestamp, self.price, self.volume,
        )
    }
}
