use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// An u8 encoded Enum that represents the unique ID of a service.
///
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum ServiceID {
    #[default]
    Default = 0x0_u8,
    QDGW = 0x1_u8,
}

impl ServiceID {
    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

impl From<u8> for ServiceID {
    /// Converts a raw byte value into a `ServiceID`.
    /// Unknown message type results in NullVal
    /// ```
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => ServiceID::Default,
            0x1_u8 => ServiceID::QDGW,
            _ => Self::Default,
        }
    }
}

impl ServiceID {
    pub fn from_string(n: &str) -> Option<ServiceID> {
        match n {
            "Default" => Some(ServiceID::Default),
            "QDGW" => Some(ServiceID::QDGW),
            _ => None,
        }
    }
}

impl Display for ServiceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceID::Default => write!(f, "Default"),
            ServiceID::QDGW => write!(f, "QDGW"),
        }
    }
}
