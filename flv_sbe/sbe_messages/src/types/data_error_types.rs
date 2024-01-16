use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(u8)]
pub enum DataErrorType {
    #[default]
    UnknownDataError = 0_u8,
    DataTypeNotKnownError = 1_u8,
    DataUnavailableError = 2_u8,
    DataEncodingError = 3_u8,
    DataTableNotFound = 4_u8,
    DataSendError = 5_u8,
}

impl From<u8> for DataErrorType {
    fn from(value: u8) -> Self {
        match value {
            0_u8 => DataErrorType::UnknownDataError,
            1_u8 => DataErrorType::DataTypeNotKnownError,
            2_u8 => DataErrorType::DataUnavailableError,
            3_u8 => DataErrorType::DataEncodingError,
            4_u8 => DataErrorType::DataTableNotFound,
            5_u8 => DataErrorType::DataSendError,
            _ => DataErrorType::UnknownDataError,
        }
    }
}

impl Display for DataErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
