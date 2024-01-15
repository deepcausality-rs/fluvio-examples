use crate::prelude::OHLCVBar;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

impl OHLCVBar {
    pub fn range_change(&self) -> Decimal {
        self.close - self.open
    }

    pub fn range_percent(&self) -> Decimal {
        let one_hundred = Decimal::new(100, 0);
        (((self.close - self.open) / self.open) * one_hundred).round_dp(4)
    }
}

impl OHLCVBar {
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

    pub fn symbol_id(&self) -> u16 {
        self.symbol_id
    }
}
