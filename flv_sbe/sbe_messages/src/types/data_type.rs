use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

/// The DataType enum represents the different data types that can be transmitted.
///
/// The variants represent the following data types:
///
/// - `UnknownDataType` - Default unknown data type
/// - `TradeData` - Trade/tick data
/// - `OHLCVData` - Open-high-low-close-volume bar data
/// - `OrderBookData` - Current order book data
/// - `QuoteData` - Quote data
///
/// The enum is represented as a `u8` under the hood.
#[derive(
Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(u8)]
pub enum DataType {
    #[default]
    UnknownDataType = 0_u8,
    TradeData = 1_u8,
    OHLCVData = 2_u8,
    OrderBookData = 3_u8,
    QuoteData = 4_u8,
}

impl From<u8> for DataType {
    /// Converts a `u8` value to a `DataType` enum variant.
    ///
    /// # Parameters
    ///
    /// * `value` - The `u8` value to convert.
    ///
    /// # Returns
    ///
    /// The corresponding `DataType` variant:
    ///
    /// - `0_u8` maps to `DataType::UnknownDataType`
    /// - `1_u8` maps to `DataType::TradeData`
    /// - `2_u8` maps to `DataType::OHLCVData`
    /// - `3_u8` maps to `DataType::OrderBookData`
    /// - `4_u8` maps to `DataType::QuoteData`
    ///
    /// Any other value maps to `DataType::UnknownDataType`.
    fn from(value: u8) -> Self {
        match value {
            0_u8 => DataType::UnknownDataType,
            1_u8 => DataType::TradeData,
            2_u8 => DataType::OHLCVData,
            3_u8 => DataType::OrderBookData,
            4_u8 => DataType::QuoteData,
            _ => DataType::UnknownDataType,
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}