use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The ExchangeID enum represents supported exchange identifiers.
///
/// The variants are:
///
/// - NullVal - A null or unset value, default variant.
/// - Kraken - The Kraken exchange.
///
/// The enum is represented as a u8 under the hood.
///
#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[repr(u8)]
pub enum ExchangeID {
    #[default]
    NullVal = 0_u8,
    Kraken = 1_u8,
}

impl From<u8> for ExchangeID {
    /// Create an ExchangeID from a u8 value.
    ///
    /// # Parameters
    ///
    /// * `v` - The u8 value to convert to an ExchangeID
    ///
    /// # Returns
    ///
    /// Returns the corresponding ExchangeID for the provided u8:
    ///
    /// - 0 -> ExchangeID::NullVal
    /// - 1 -> ExchangeID::Kraken
    ///
    /// If the u8 does not match a valid mapping, returns ExchangeID::NullVal.
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0 => Self::NullVal,
            1 => Self::Kraken,
            _ => Self::NullVal,
        }
    }
}

impl Display for ExchangeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExchangeID::NullVal => write!(f, "NullVal"),
            ExchangeID::Kraken => write!(f, "Kraken"),
        }
    }
}
