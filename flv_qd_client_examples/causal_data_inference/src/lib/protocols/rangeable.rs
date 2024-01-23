
use deep_causality::prelude::Datable;

use crate::types::bar_range::BarRange;

/// The Rangeable trait defines the behavior for types that can provide a data range.
///
/// This trait requires that the implementor also implements the Datable trait.
///
/// # Methods
///
/// * `data_range(&self) -> BarRange` - Returns a BarRange representing the data range for this value.
///     The BarRange contains the high, low, open, and close values.
///
pub trait Rangeable: Datable {
    fn data_range(&self) -> BarRange;
}