use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum AccountType {
    NullVal = 0xff_u8,
    #[default]
    Spot = 0x1_u8,
    Margin = 0x2_u8,
    Future = 0x3_u8,
}

impl From<i32> for AccountType {
    #[inline]
    fn from(v: i32) -> Self {
        match v {
            0x1_i32 => Self::Spot,
            0x2_i32 => Self::Margin,
            0x3_i32 => Self::Future,
            _ => Self::NullVal,
        }
    }
}

impl Display for AccountType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::NullVal => write!(f, "NullVal"),
            AccountType::Spot => write!(f, "Spot"),
            AccountType::Margin => write!(f, "Margin"),
            AccountType::Future => write!(f, "Future"),
        }
    }
}
