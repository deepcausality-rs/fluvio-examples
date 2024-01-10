use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Enum representing the different types of messages that can be sent over network.
#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(u8)]
pub enum ClientErrorType {
    #[default]
    UnknownClientError = 0_u8,
    ClientAlreadyLoggedIn = 1_u8,
    ClientLogInError = 2_u8,
    ClientNotLoggedIn = 3_u8,
    ClientLogOutError = 4_u8,
}

impl From<u8> for ClientErrorType {
    fn from(value: u8) -> Self {
        match value {
            0_u8 => ClientErrorType::UnknownClientError,
            1_u8 => ClientErrorType::ClientAlreadyLoggedIn,
            2_u8 => ClientErrorType::ClientLogInError,
            3_u8 => ClientErrorType::ClientNotLoggedIn,
            4_u8 => ClientErrorType::ClientLogOutError,
            _ => ClientErrorType::UnknownClientError,
        }
    }
}

impl Display for ClientErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
