use serde::{Deserialize, Serialize};
use std::fmt;

/// The TimeResolution enum represents different time resolutions that can be used.
///
/// It has the following variants:
///
/// - `NoValue`: Default value representing no time resolution specified.
/// - `OneMin`: 1 minute time resolution.
/// - `FiveMin`: 5 minute time resolution.
/// - `FifteenMin`: 15 minute time resolution.
/// - `ThirtyMin`: 30 minute time resolution.
/// - `OneHour`: 1 hour time resolution.
/// - `OneDay`: 1 day time resolution.
/// - `OneMonth`: 1 month time resolution.
/// - `OneYear`: 1 year time resolution.
///
/// The enum is decorated with various attributes like `Serialize`, `Deserialize`, etc.
/// to control how it is (de)serialized. It is also represented as a `u8` under the hood.
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum TimeResolution {
    #[default]
    NoValue = 0_u8,
    OneMin = 1_u8,
    FiveMin = 2_u8,
    FifteenMin = 3_u8,
    ThirtyMin = 4_u8,
    OneHour = 5_u8,
    OneDay = 6_u8,
    OneWeek = 7_u8,
    OneMonth = 8_u8,
    OneYear = 9_u8,
}

impl From<u8> for TimeResolution {
    /// Implements the From trait to convert a u8 to a TimeResolution.
    ///
    /// Matches the u8 value to the corresponding TimeResolution variant:
    ///
    /// 0 -> NoValue
    /// 1 -> OneMin
    /// 2 -> FiveMin
    /// etc.
    ///
    /// # Arguments
    ///
    /// * `v` - u8 value to convert
    ///
    /// # Returns
    ///
    /// TimeResolution variant
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0_u8 => Self::NoValue,
            1_u8 => Self::OneMin,
            2_u8 => Self::FiveMin,
            3_u8 => Self::FifteenMin,
            4_u8 => Self::ThirtyMin,
            5_u8 => Self::OneHour,
            6_u8 => Self::OneDay,
            7_u8 => Self::OneWeek,
            8_u8 => Self::OneMonth,
            9_u8 => Self::OneYear,
            _ => Self::NoValue,
        }
    }
}

impl fmt::Display for TimeResolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TimeResolution::NoValue => write!(f, "NoValue"),
            TimeResolution::OneMin => write!(f, "1 minute"),
            TimeResolution::FiveMin => write!(f, "5 minute"),
            TimeResolution::FifteenMin => write!(f, "15 minute"),
            TimeResolution::ThirtyMin => write!(f, "30 minute"),
            TimeResolution::OneHour => write!(f, "1 hour"),
            TimeResolution::OneDay => write!(f, "1 day"),
            TimeResolution::OneWeek => write!(f, "1 week"),
            TimeResolution::OneMonth => write!(f, "1 month"),
            TimeResolution::OneYear => write!(f, "1 year"),
        }
    }
}
