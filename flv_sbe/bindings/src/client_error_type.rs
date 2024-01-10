#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ClientErrorType {
    UnknownClientError = 0x0_u8,
    ClientAlreadyLoggedIn = 0x1_u8,
    ClientLogInError = 0x2_u8,
    ClientNotLoggedIn = 0x3_u8,
    ClientLogOutError = 0x4_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for ClientErrorType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::UnknownClientError,
            0x1_u8 => Self::ClientAlreadyLoggedIn,
            0x2_u8 => Self::ClientLogInError,
            0x3_u8 => Self::ClientNotLoggedIn,
            0x4_u8 => Self::ClientLogOutError,
            _ => Self::NullVal,
        }
    }
}
