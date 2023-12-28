use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub enum SecurityType {
    UnknownSecurityType,
    #[default]
    Spot,
    Index,
    Future,
    PerpetualFuture,
    Option,
    FutureOption,
}

impl Display for SecurityType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityType::UnknownSecurityType => write!(f, "UnknownSecurityType"),
            SecurityType::Spot => write!(f, "Spot"),
            SecurityType::Index => write!(f, "Index"),
            SecurityType::Future => write!(f, "Future"),
            SecurityType::PerpetualFuture => write!(f, "PerpetualFuture"),
            SecurityType::Option => write!(f, "Option"),
            SecurityType::FutureOption => write!(f, "FutureOption"),
        }
    }
}
