use deep_causality::prelude::Datable;

use crate::types::bar_range::BarRange;

// Extension trait http://xion.io/post/code/rust-extension-traits.html

/// The Rangeable trait defines the behavior for types that can provide a data range.
///
/// This trait requires that the implementor also implements the Datable trait.
///
/// # Methods
///
/// * `data_range(&self) -> BarRange` - Returns a BarRange representing the data range for this value.
///     The BarRange contains the following fields:
///
/// * `high` - The highest price during the bar interval
/// * `close` - The closing price at the end of the bar interval
/// * `close_above_open` - Whether the closing price is above the opening price
/// * `close_below_open` - Whether the closing price is below the opening price
///
pub trait RangeExt: Datable {
    fn data_range(&self) -> BarRange;
}
