use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

/// The DataType enum represents the different data types that can be encoded and decoded.
///
/// The variants are:
///
/// - TradeData - Encodes trade data with a value of 0.
/// - OHLCVData - Encodes OHLCV (Open, High, Low, Close, Volume) data with a value of 1.
/// - OrderBookData - Encodes order book data with a value of 2.
/// - QuoteData - Encodes quote data with a value of 4.
///
/// The enum is marked with #[repr(u8)] for serialization as a u8.
#[derive(
Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(u8)]
pub enum DataType {
    UnknownDataType = 0_u8,
    TradeData = 1_u8,
    OHLCVData = 2_u8,
    OrderBookData = 3_u8,
    QuoteData = 4_u8,
}

impl From<u8> for DataType {
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