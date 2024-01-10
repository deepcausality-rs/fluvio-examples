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
    ExecutionChannel = 2,
    HeartbeatChannel = 3,
}

impl fmt::Display for ClientChannel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClientChannel::DataChannel => write!(f, "DataChannel"),
            ClientChannel::ControlChannel => write!(f, "ControlChannel"),
            ClientChannel::ExecutionChannel => write!(f, "ExecutionChannel"),
            ClientChannel::HeartbeatChannel => write!(f, "HeartbeatChannel"),
        }
    }
}
