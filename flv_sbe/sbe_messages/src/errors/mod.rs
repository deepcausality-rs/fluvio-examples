use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SbeEncodeError(pub String);

impl fmt::Display for SbeEncodeError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SbeEncodeError: {}", self.0)
    }
}

impl std::error::Error for SbeEncodeError {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SbeDecodeError(pub String);

impl std::error::Error for SbeDecodeError {}

impl fmt::Display for SbeDecodeError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SbeDecodeError: {}", self.0)
    }
}
