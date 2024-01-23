use std::fmt::{Display, Formatter};

use rust_decimal::Decimal;

/// The BarRange struct represents the price range for a single bar/candlestick.
///
/// It contains the high, low, open and close prices for the bar interval.
///
/// # Fields
///
/// * `high` - The highest price during the bar interval
/// * `close` - The closing price at the end of the bar interval
/// * `close_above_open` - Whether the closing price is above the opening price
/// * `close_below_open` - Whether the closing price is below the opening price
///
/// # Methods
///
/// * `new` - Construct a new BarRange instance
/// * `high` - Get the high price
/// * `close` - Get the close price
/// * `close_above_open` - Check if close is above open
/// * `close_below_open` - Check if close is below open
/// * `fmt` - Format the BarRange as a string for display
///
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct BarRange {
    high: Decimal,
    close: Decimal,
    close_above_open: bool,
    close_below_open: bool,
}

impl BarRange {
    pub fn new(
        high: Decimal,
        close: Decimal,
        close_above_open: bool,
        close_below_open: bool,
    ) -> Self {
        Self {
            high,
            close,
            close_above_open,
            close_below_open,
        }
    }
}

impl BarRange {
    pub fn high(&self) -> Decimal {
        self.high
    }
    pub fn close(&self) -> Decimal {
        self.close
    }
    pub fn close_above_open(&self) -> bool {
        self.close_above_open
    }
    pub fn close_below_open(&self) -> bool {
        self.close_below_open
    }
}

impl Display for BarRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BarRange {{ high: {}, close: {}, close_above_open: {}, close_below_open: {} }}",
            self.high, self.close, self.close_above_open, self.close_below_open
        )
    }
}
