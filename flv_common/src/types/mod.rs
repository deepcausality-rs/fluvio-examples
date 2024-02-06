/// Common data type definitions
///
/// This crate provides common data types used throughout the application.
///
/// It contains the following modules:
///
/// - `config_types`: Configuration data structures.
/// - `data_types`: Core domain data types.
/// - `error_types`: Error types.
/// - `exchange_types`: Exchange metadata types.
/// - `symbol_types`: Symbol types.
/// - `time_types`: Time and date types.
///
/// The goal is to centralize key data types for consistency. Other
/// crates can import just the types they need.
///
/// Using common types improves interoperability and reduces duplication.
/// The types provide validation, serialization, and other functionality.
///
pub mod config_types;
pub mod data_types;

pub mod error_types;
pub mod exchange_types;
pub mod symbol_types;
pub mod time_types;
