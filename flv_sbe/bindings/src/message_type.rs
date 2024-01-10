#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum MessageType {
    UnknownMessageType = 0x0_u16, 
    ClientLogin = 0x65_u16, 
    ClientLogout = 0x66_u16, 
    StartData = 0xc9_u16, 
    StopData = 0xca_u16, 
    StopAllData = 0xcb_u16, 
    DataBar = 0xcc_u16, 
    FirstDataBar = 0xcd_u16, 
    LastDataBar = 0xce_u16, 
    ClientError = 0x321_u16, 
    #[default]
    NullVal = 0xffff_u16, 
}
impl From<u16> for MessageType {
    #[inline]
    fn from(v: u16) -> Self {
        match v {
            0x0_u16 => Self::UnknownMessageType, 
            0x65_u16 => Self::ClientLogin, 
            0x66_u16 => Self::ClientLogout, 
            0xc9_u16 => Self::StartData, 
            0xca_u16 => Self::StopData, 
            0xcb_u16 => Self::StopAllData, 
            0xcc_u16 => Self::DataBar, 
            0xcd_u16 => Self::FirstDataBar, 
            0xce_u16 => Self::LastDataBar, 
            0x321_u16 => Self::ClientError, 
            _ => Self::NullVal,
        }
    }
}
