use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum EnvironmentType {
    #[default]
    Local = 0x0_u8,
    Cluster = 0x1_u8,
}

impl From<u8> for EnvironmentType {
    fn from(value: u8) -> Self {
        match value {
            0x0_u8 => EnvironmentType::Local,
            0x1_u8 => EnvironmentType::Cluster,
            _ => panic!("Invalid environment type value: {}", value),
        }
    }
}

impl EnvironmentType {
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
