/// Module containing common data types used across SBE messages.
///
/// This includes enums representing:  
///
/// - Data types like trades, OHLCV etc.
/// - Data errors for requests
/// - Exchange identifiers
/// - Message types
///
/// Grouping these common types into a module avoids duplication
/// and provides a single source of truth for type definitions.
///
/// The types are exposed in the prelude so they can be conveniently
/// imported together via `use sbe_messages::prelude::*`.
///
/// # Exports
///
/// - `data_type` - Enumeration of data types
/// - `data_error_types` - Enumeration of data error types
/// - `exchange_id` - Enumeration of exchange identifiers
/// - `message_type` - Enumeration of message types
///
pub mod client_error_types;
pub mod data_error_types;
pub mod data_type;
pub mod message_types;
