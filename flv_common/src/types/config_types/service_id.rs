use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// ServiceID enum definition.
///
/// Encodes the unique ID of a service as a u8.
///
/// # Variants
///
/// `Default` - Default value, ID = 0
/// `Database` - Database service, ID = 99
/// `QDGW` - Quote Data Gateway service, ID = 1
/// `SYMDB` - Symbol Master Service, ID = 2
///
/// # Implements
///
/// `Serialize`, `Deserialize` - Serde serialization
/// `Debug`, `Default`, `Copy`, `Clone`, `Eq`, `PartialEq` - Rust defaults
/// `Display` - Custom Display impl
///
/// # Notes
///
/// `#[repr(u8)]` encodes as u8 for serialization.
///
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum ServiceID {
    #[default]
    Default = 0x0_u8,
    Database = 99_u8,
    QDGW = 0x1_u8,
    SYMDB = 0x2_u8,
}

impl ServiceID {
    /// Returns the raw u8 ID value for the ServiceID variant.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Returns
    ///
    /// `u8` - The raw enum variant value cast to u8
    pub fn id(&self) -> u8 {
        *self as u8
    }

    /// Returns the string name for the ServiceID variant.
    ///
    /// Calls to_string() to convert the Display impl to a String.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Returns
    ///
    /// `String` - The string name of the variant
    pub fn name(&self) -> String {
        self.to_string()
    }
}

impl From<u8> for ServiceID {
    /// Implements the From trait to convert a u8 to a ServiceID.
    ///
    /// Matches on the u8 value:
    ///
    /// 0x0 -> Default
    /// 0x1 -> QDGW
    ///
    /// Unknown values default to Default.
    ///
    /// # Arguments
    ///
    /// * `v` - u8 value to convert
    ///
    /// # Returns
    ///
    /// ServiceID variant
    ///
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => ServiceID::Default,
            0x1_u8 => ServiceID::QDGW,
            0x2_u8 => Self::SYMDB,
            99_u8 => Self::Database,
            _ => Self::Default,
        }
    }
}

impl ServiceID {
    /// Converts a string to a ServiceID variant.
    ///
    /// Matches on the string value:
    ///
    /// "Default" -> Default
    /// "QDGW" -> QDGW
    ///
    /// Unknown strings return None.
    ///
    /// # Arguments
    ///
    /// * `n` - String to convert
    ///
    /// # Returns
    ///
    /// `Option<ServiceID>` - Some(variant) on match, None if no match
    ///
    #[inline]
    pub fn from_string(n: &str) -> Option<ServiceID> {
        match n {
            "Default" => Some(ServiceID::Default),
            "QDGW" => Some(ServiceID::QDGW),
            "SYMDB" => Some(ServiceID::SYMDB),
            "Database" => Some(ServiceID::Database),
            _ => None,
        }
    }
}

impl Display for ServiceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceID::Default => write!(f, "Default"),
            ServiceID::Database => write!(f, "Database"),
            ServiceID::QDGW => write!(f, "QDGW"),
            ServiceID::SYMDB => write!(f, "SYMDB"),
        }
    }
}
