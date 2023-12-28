use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum TimeResolution {
    #[default]
    NoValue = 0x0_u8,
    OneMin = 0x1_u8,
    FiveMin = 0x2_u8,
    FifteenMin = 0x3_u8,
    ThirtyMin = 0x4_u8,
    OneHour = 0x5_u8,
    OneDay = 0x6_u8,
    OneWeek = 0x7_u8,
}

impl FromStr for TimeResolution {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NoValue" => Ok(TimeResolution::NoValue),
            "OneMin" => Ok(TimeResolution::OneMin),
            "FiveMin" => Ok(TimeResolution::FiveMin),
            "FifteenMin" => Ok(TimeResolution::FifteenMin),
            "ThirtyMin" => Ok(TimeResolution::ThirtyMin),
            "OneHour" => Ok(TimeResolution::OneHour),
            "OneDay" => Ok(TimeResolution::OneDay),
            "OneWeek" => Ok(TimeResolution::OneWeek),
            _ => Err(()),
        }
    }
}

impl From<u8> for TimeResolution {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::NoValue,
            0x1_u8 => Self::OneMin,
            0x2_u8 => Self::FiveMin,
            0x3_u8 => Self::FifteenMin,
            0x4_u8 => Self::ThirtyMin,
            0x5_u8 => Self::OneHour,
            0x6_u8 => Self::OneDay,
            0x7_u8 => Self::OneWeek,
            _ => Self::NoValue,
        }
    }
}

impl fmt::Display for TimeResolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TimeResolution::NoValue => write!(f, "NoValue"),
            TimeResolution::OneMin => write!(f, "OneMin"),
            TimeResolution::FiveMin => write!(f, "FiveMin"),
            TimeResolution::FifteenMin => write!(f, "FifteenMin"),
            TimeResolution::ThirtyMin => write!(f, "ThirtyMin"),
            TimeResolution::OneHour => write!(f, "OneHour"),
            TimeResolution::OneDay => write!(f, "OneDay"),
            TimeResolution::OneWeek => write!(f, "OneWeek"),
        }
    }
}
