use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ExchangeID {
    #[default]
    NullVal = 0_u8,
    BinanceSpot = 1_u8,
    COINBASE = 2_u8,
    VEX = 3_u8,
}

impl From<u8> for ExchangeID {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0 => Self::NullVal,
            1 => Self::BinanceSpot,
            2 => Self::COINBASE,
            3 => Self::VEX,
            _ => Self::NullVal,
        }
    }
}

impl Display for ExchangeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExchangeID::NullVal => write!(f, "NullVal"),
            ExchangeID::BinanceSpot => write!(f, "BinanceSpot"),
            ExchangeID::COINBASE => write!(f, "COINBASE"),
            ExchangeID::VEX => write!(f, "VEX"),
        }
    }
}
