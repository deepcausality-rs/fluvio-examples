use std::fmt;

/// The ClientChannel enum represents the different channels a client can use.
///
/// This is represented as a u8 under the hood and includes the following variants:
///
/// - DataChannel - The channel for sending data payloads. This has the underlying value 0.
///
/// - ControlChannel - The channel for sending control messages. This has the underlying value 1.
///
/// - ExecutionChannel - The channel for sending execution messages. This has the underlying value 2.
///
/// - HeartbeatChannel - The channel for sending heartbeat messages. This has the underlying value 3.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ClientChannel {
    DataChannel = 0,
    ControlChannel = 1,
    ErrorChannel = 2,
    ExecutionChannel = 3,
    HeartbeatChannel = 4,
}

impl From<u8> for ClientChannel {
    /// Implements the From trait to convert a u8 to a ClientChannel.
    ///
    /// Matches on the u8 value:
    ///
    /// 0 -> DataChannel
    /// 1 -> ControlChannel
    /// 2 -> ErrorChannel
    /// 3 -> ExecutionChannel
    /// 4 -> HeartbeatChannel
    ///
    /// Panics on unknown value.
    ///
    /// # Arguments
    ///
    /// * `value` - u8 value to convert
    ///
    /// # Returns
    ///
    /// ClientChannel variant
    ///
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => ClientChannel::DataChannel,
            1 => ClientChannel::ControlChannel,
            2 => ClientChannel::ErrorChannel,
            3 => ClientChannel::ExecutionChannel,
            4 => ClientChannel::HeartbeatChannel,
            _ => panic!("Unknown ClientChannel value: {}", value),
        }
    }
}

impl fmt::Display for ClientChannel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClientChannel::DataChannel => write!(f, "DataChannel"),
            ClientChannel::ControlChannel => write!(f, "ControlChannel"),
            ClientChannel::ErrorChannel => write!(f, "ErrorChannel"),
            ClientChannel::ExecutionChannel => write!(f, "ExecutionChannel"),
            ClientChannel::HeartbeatChannel => write!(f, "HeartbeatChannel"),
        }
    }
}
