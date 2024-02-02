use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct SymdbClientError(pub String);

impl Error for SymdbClientError {}

impl Display for SymdbClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SymdbClientError: {}", self.0)
    }
}
