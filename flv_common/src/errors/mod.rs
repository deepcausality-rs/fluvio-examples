use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct InitError(pub String);

impl Error for InitError {}

impl fmt::Display for InitError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InitError: {}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct MessageProcessingError(pub String);

impl Error for MessageProcessingError {}

impl fmt::Display for MessageProcessingError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MessageProcessingError: {}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct MessageClientConfigError(pub String);

impl Error for MessageClientConfigError {}

impl fmt::Display for MessageClientConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MessageClientConfigError: {}", self.0)
    }
}

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
