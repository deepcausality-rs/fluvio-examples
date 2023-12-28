#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MessageType {
    UnknownMessageType = 0x0_u8, 
    ClientLogin = 0x1_u8, 
    ClientLogout = 0x2_u8, 
    StartData = 0x3_u8, 
    StopData = 0x4_u8, 
    StopAllData = 0x5_u8, 
    DataBar = 0x6_u8, 
    LastDataBar = 0x7_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for MessageType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::UnknownMessageType, 
            0x1_u8 => Self::ClientLogin, 
            0x2_u8 => Self::ClientLogout, 
            0x3_u8 => Self::StartData, 
            0x4_u8 => Self::StopData, 
            0x5_u8 => Self::StopAllData, 
            0x6_u8 => Self::DataBar, 
            0x7_u8 => Self::LastDataBar, 
            _ => Self::NullVal,
        }
    }
}
