use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// An u8 encoded Enum that represents the unique ID of a service.
///
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum ServiceID {
    #[default]
    Default = 0x0_u8,
    DBGW = 0x1_u8,
    QDGW = 0x2_u8,
    VEX = 0x3_u8,
}

impl ServiceID {
    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

impl From<i32> for ServiceID {
    /// Converts a raw byte value into a `ServiceID`.
    /// Unknown message type results in NullVal
    /// ```
    #[inline]
    fn from(v: i32) -> Self {
        match v {
            0x0_i32 => ServiceID::Default,
            0x1_i32 => ServiceID::DBGW,
            0x2_i32 => ServiceID::QDGW,
            0x3_i32 => ServiceID::VEX,
            _ => Self::Default,
        }
    }
}

impl ServiceID {
    pub fn from_string(n: &str) -> Option<ServiceID> {
        match n {
            "Default" => Some(ServiceID::Default),
            "DBGW" => Some(ServiceID::DBGW),
            "QDGW" => Some(ServiceID::QDGW),
            "VEX" => Some(ServiceID::VEX),
            _ => None,
        }
    }
}

impl Display for ServiceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceID::Default => write!(f, "Default"),
            ServiceID::DBGW => write!(f, "DBGW"),
            ServiceID::QDGW => write!(f, "QDGW"),
            ServiceID::VEX => write!(f, "VEX"),
        }
    }
}
