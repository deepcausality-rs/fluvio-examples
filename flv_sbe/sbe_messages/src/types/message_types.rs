use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

/// Enum representing the different types of messages that can be sent over network.
#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(u8)]
pub enum MessageType {
    #[default]
    UnknownMessageType = 0xff_u8,
    ClientLogin = 0x1_u8,
    ClientLogout = 0x2_u8,
    StartData = 0x3_u8,
    StopData = 0x4_u8,
    StopAllData = 0x5_u8,
    DataBar = 0x6_u8,
    LastDataBar = 0x7_u8,
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0x1 => MessageType::ClientLogin,
            0x2 => MessageType::ClientLogout,
            0x3 => MessageType::StartData,
            0x4 => MessageType::StopData,
            0x5 => MessageType::StopAllData,
            0x6 => MessageType::DataBar,
            0x7 => MessageType::LastDataBar,
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
            MessageType::LastDataBar => write!(f, "LastDataBar"),
            MessageType::UnknownMessageType => write!(f, "UnknownMessageType"),
        }
    }
}
