use crate::prelude::{BarRange, Rangeable};
use deep_causality::prelude::{Datable, Identifiable};
use std::fmt::{Display, Formatter};

/// The CustomData struct represents custom data with an identifier and data range.
///
/// It implements several traits to provide the required functionality:
///
/// - Datable - Marks this as data that can be used in causal inference
/// - Identifiable - Provides a unique ID for this data
/// - Rangeable - Provides a price range (BarRange) for this data
/// - Display - Allows formatting the data as a string
///
/// # Fields
///
/// * `id` - A unique identifier for this data
/// * `data_range` - The price range for this data as a BarRange
///
/// # Methods
///
/// * `new` - Constructs a new CustomData instance
/// * `id` - Returns the unique ID for this data
/// * `data_range` - Returns the BarRange representing the price range
/// * `fmt` - Formats the data as a string for display
///
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct RangeData {
    id: u64,
    data_range: BarRange,
}

impl RangeData {
    pub fn new(id: u64, data_range: BarRange) -> Self {
        Self { id, data_range }
    }
}

impl Datable for RangeData {}

impl Identifiable for RangeData {
    fn id(&self) -> u64 {
        self.id
    }
}

impl Rangeable for RangeData {
    fn data_range(&self) -> BarRange {
        self.data_range
    }
}

impl Display for RangeData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {} range: {}", self.id, self.data_range)
    }
}
