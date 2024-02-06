use std::error::Error;
use std::fmt;

/// InitError custom error type.
///
/// Contains a single String field to hold the error message.
///
/// # Fields
///
/// `String` - The error message
///
/// # Implements
///
/// `Debug` - Formatted debug output
/// `Clone` - Clone support
/// `Error` - std::error::Error impl
/// `Display` - Formatted display output
///
#[derive(Debug, Clone)]
pub struct InitError(pub String);

impl InitError {
    pub fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl Error for InitError {}

impl fmt::Display for InitError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InitError: {}", self.0)
    }
}

/// LookupError custom error type.
///
/// Contains a single String field to hold the error message.
///
/// # Fields
///
/// `String` - The error message
///
/// # Implements
///
/// `Debug` - Formatted debug output
/// `Clone` - Clone support
/// `Error` - std::error::Error impl
/// `Display` - Formatted display output
///
#[derive(Debug, Clone)]
pub struct LookupError(pub String);

impl LookupError {
    pub fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl Error for LookupError {}

impl fmt::Display for LookupError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LookupError: {}", self.0)
    }
}

/// MessageProcessingError custom error type.
///
/// Contains a single String field to hold the error message.
///
/// # Fields
///
/// `String` - The error message
///
/// # Implements
///
/// `Debug` - Formatted debug output
/// `Clone` - Clone support
/// `Error` - std::error::Error impl
/// `Display` - Formatted display output
//
#[derive(Debug, Clone)]
pub struct MessageProcessingError(pub String);

impl Error for MessageProcessingError {}

impl fmt::Display for MessageProcessingError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MessageProcessingError: {}", self.0)
    }
}

/// MessageClientConfigError custom error type.
///
/// Contains a single String field to hold the error message.
///
/// # Fields
///
/// `String` - The error message
///
/// # Implements
///
/// `Debug` - Formatted debug output
/// `Clone` - Clone support
/// `Error` - std::error::Error impl
/// `Display` - Formatted display output
///
#[derive(Debug, Clone)]
pub struct MessageClientConfigError(pub String);

impl Error for MessageClientConfigError {}

impl fmt::Display for MessageClientConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MessageClientConfigError: {}", self.0)
    }
}

/// ValidationError custom error type.
///
/// Contains a single String field to hold the error message.
///
/// # Fields
///
/// `String` - The error message
///
/// # Implements
///
/// `Debug` - Formatted debug output
/// `Clone` - Clone support
/// `Error` - std::error::Error impl
/// `Display` - Formatted display output
///
#[derive(Debug, Clone)]
pub struct ValidationError(pub String);

impl ValidationError {
    pub fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl Error for ValidationError {}

impl fmt::Display for ValidationError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ValidationError: {}", self.0)
    }
}
