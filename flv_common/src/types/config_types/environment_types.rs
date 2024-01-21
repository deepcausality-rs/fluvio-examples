use serde::{Deserialize, Serialize};
use std::fmt;

/// EnvironmentType enum definition.
///
/// Encodes the environment type as a u8.
///
/// # Variants
///
/// `Local` - Local development environment, ID = 0x0
/// `Cluster` - Kubernetes cluster environment, ID = 0x1
///
/// # Implements
///
/// `Serialize`, `Deserialize` - Serde serialization
/// `Debug`, `Default`, `Copy`, `Clone`, `Eq`, `PartialEq` - Rust defaults
///
/// # Notes
///
/// `#[repr(u8)]` encodes as u8 for serialization.
///
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum EnvironmentType {
    #[default]
    Local = 0x0_u8,
    Cluster = 0x1_u8,
}

impl From<u8> for EnvironmentType {
    /// Implements the From trait to convert a u8 to an EnvironmentType.
    ///
    /// Matches on the u8 value:
    ///
    /// 0x0 -> Local
    /// 0x1 -> Cluster
    ///
    /// Panics on invalid value.
    ///
    /// # Arguments
    ///
    /// * `value` - u8 value to convert
    ///
    /// # Returns
    ///
    /// EnvironmentType variant
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0x0_u8 => EnvironmentType::Local,
            0x1_u8 => EnvironmentType::Cluster,
            _ => panic!("Invalid environment type value: {}", value),
        }
    }
}

impl EnvironmentType {
    /// Converts a string to an EnvironmentType.
    ///
    /// Matches on the string value:
    ///
    /// "local" -> Local
    /// "k8s" -> Cluster
    ///
    /// Returns Err on invalid string.
    ///
    /// # Arguments
    ///
    /// * `s` - String to convert
    ///
    /// # Returns
    ///
    /// `Result<EnvironmentType, &'static str>` - Ok(variant) on match, Err on invalid string
    ///
    #[inline]
    pub fn from_string(s: &str) -> Result<Self, &'static str> {
        match s {
            "local" => Ok(Self::Local),
            "k8s" => Ok(Self::Cluster),
            _ => Err("Invalid environment type string"),
        }
    }
}

impl fmt::Display for EnvironmentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EnvironmentType::Local => write!(f, "Local"),
            EnvironmentType::Cluster => write!(f, "Cluster"),
        }
    }
}
