//! Module containing SBE message definitions.
//!
//! This includes modules grouping related message types:
//!
//! - `client_messages` - Messages from clients
//! - `data_messages` - Data subscription messages
//! - `error_messages` - Error response messages
//!
//! Grouping messages into modules provides organization and encapsulation.
//!
//! The messages are exposed in the prelude for convenient importing.
//!
pub mod client_messages;
pub mod data_messages;
pub mod error_messages;
