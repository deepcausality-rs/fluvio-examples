use std::fmt;

/// SbeEncodeError struct definition.
///
/// Used to represent SBE encoding errors.
///
/// # Fields
///
/// `0` - Error message string
///
/// # Implements
///
/// `Clone`, `Debug`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash` - Rust defaults
/// `fmt::Display` - Custom Display implementation to print error messages
/// `std::error::Error` - Implements std::error::Error trait
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SbeEncodeError(pub String);

impl fmt::Display for SbeEncodeError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SbeEncodeError: {}", self.0)
    }
}

impl std::error::Error for SbeEncodeError {}

/// SbeDecodeError struct definition.
///
/// Used to represent SBE decoding errors.
///
/// # Fields
///
/// `0` - Error message string
///
/// # Implements
///
/// `Clone`, `Debug`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash` - Rust defaults
/// `fmt::Display` - Custom Display implementation to print error messages
/// `std::error::Error` - Implements std::error::Error trait
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SbeDecodeError(pub String);

impl std::error::Error for SbeDecodeError {}

impl fmt::Display for SbeDecodeError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SbeDecodeError: {}", self.0)
    }
}
