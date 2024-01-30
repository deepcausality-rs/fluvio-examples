use common::prelude::ValidationError;
use std::error::Error;
use std::fmt;

/// Custom error type for DB query errors
#[derive(Debug)]
pub enum QueryError {
    QueryFailed(String),
    InvalidTableName(ValidationError),
    EmptyTableName(ValidationError),
    TableNameTooLong(ValidationError),
}

impl Error for QueryError {}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueryError::QueryFailed(e) =>
                write!(f,
                       "Query to DB failed: {}",
                       e
                ),

            QueryError::InvalidTableName(e) =>
                write!(f,
                       "Invalid table name provided: Only use alphanumeric characters and underscores as table name. Error: {}",
                       e,
                ),

            QueryError::EmptyTableName(e) =>
                write!(f,
                       "Empty table name provided: Table must have a name. Error: {}",
                       e
                ),

            QueryError::TableNameTooLong(e) =>
                write!(f,
                       "Table name exceeds maximum length: Table can only be 63 characters long. Error: {}",
                       e
                ),
        }
    }
}
