use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(u16)] // Type is u16 for direct conversion from proto integer. The smallest possible integer in proto is 16B.
pub enum SymbolID {
    #[default]
    NullVal = 0xff_u16,
    BTCUSD = 0x1_u16,
    ETHUSD = 0x2_u16,
    LTCUSD = 0x3_u16,
}

impl From<u16> for SymbolID {
    #[inline]
    fn from(v: u16) -> Self {
        match v {
            0xff_u16 => SymbolID::NullVal,
            0x1_u16 => SymbolID::BTCUSD,
            0x2_u16 => SymbolID::ETHUSD,
            0x3_u16 => SymbolID::LTCUSD,
            _ => Self::NullVal,
        }
    }
}

impl SymbolID {
    pub fn from_string(s: &str) -> SymbolID {
        match s.to_lowercase().as_str() {
            "btcusd" => SymbolID::BTCUSD,
            "ethusd" => SymbolID::ETHUSD,
            "ltcusd" => SymbolID::LTCUSD,
            _ => SymbolID::NullVal,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            SymbolID::BTCUSD => "btcusd",
            SymbolID::ETHUSD => "ethusd",
            SymbolID::LTCUSD => "ltcusd",
            _ => "NullVal",
        }
    }
}

impl Display for SymbolID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolID::NullVal => write!(f, "NullVal"),
            SymbolID::BTCUSD => write!(f, "BTCUSD"),
            SymbolID::ETHUSD => write!(f, "ETHUSD"),
            SymbolID::LTCUSD => write!(f, "LTCUSD"),
        }
    }
}
