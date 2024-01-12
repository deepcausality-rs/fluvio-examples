use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

/// Enum representing the different types of messages that can be sent over network.
#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(u16)]
pub enum MessageType {
    #[default]
    UnknownMessageType = 0_u16,
    // Client Message Types
    ClientLogin = 101_u16,
    ClientLogout = 102_u16,
    // Data Message Types
    StartData = 201_u16,
    StopData = 202_u16,
    StopAllData = 203_u16,
    DataBar = 204_u16,
    FirstDataBar = 205_u16,
    LastDataBar = 206_u16,
    TradeBar = 207_u16,
    FirstTradeBar = 208_u16,
    LastTradeBar = 209_u16,
    // Error Message Types
    ClientError = 801_u16,
    DataError = 802_u16,
}

impl From<u16> for MessageType {
    fn from(value: u16) -> Self {
        match value {
            0_u16 => MessageType::UnknownMessageType,
            // Client Message Types
            101_u16 => MessageType::ClientLogin,
            102_u16 => MessageType::ClientLogout,
            // Data Message Types
            201_u16 => MessageType::StartData,
            202_u16 => MessageType::StopData,
            203_u16 => MessageType::StopAllData,
            204_u16 => MessageType::DataBar,
            205_u16 => MessageType::FirstDataBar,
            206_u16 => MessageType::LastDataBar,
            207_u16 => MessageType::TradeBar,
            208_u16 => MessageType::FirstTradeBar,
            209_u16 => MessageType::LastTradeBar,
            // Error Message Types
            801_u16 => MessageType::ClientError,
            802_u16 => MessageType::DataError,
            _ => MessageType::UnknownMessageType,
        }
    }
}

impl Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageType::ClientLogin => write!(f, "ClientLogin"),
            MessageType::ClientLogout => write!(f, "ClientLogout"),
            MessageType::StartData => write!(f, "StartData"),
            MessageType::StopData => write!(f, "StopData"),
            MessageType::StopAllData => write!(f, "StopAllData"),
            MessageType::DataBar => write!(f, "DataBar"),
            MessageType::FirstDataBar => write!(f, "FirstDataBar"),
            MessageType::LastDataBar => write!(f, "LastDataBar"),
            MessageType::TradeBar => write!(f, "TradeBar"),
            MessageType::FirstTradeBar => write!(f, "FirstTradeBar"),
            MessageType::LastTradeBar => write!(f, "LastTradeBar"),
            MessageType::UnknownMessageType => write!(f, "UnknownMessageType"),
            MessageType::ClientError => write!(f, "ClientError"),
            MessageType::DataError => write!(f, "DataError"),
        }
    }
}
