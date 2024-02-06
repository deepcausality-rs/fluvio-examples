/// Exchange metadata type definitions
///
/// This module provides types to represent metadata related to exchanges.
///
/// The types provided include:
///
/// - `AccountType`: The type of exchange account.
/// - `ExchangeID`: Identifier for an exchange.
/// - `SecurityType`: The security type like spot, futures, etc.
///
/// These types are used in messages and data types to provide additional
/// context and metadata related to exchanges. For example, associating a
/// trade message with the exchange it came from.
///
/// The types are simple Rust enums represented as u8s. They define
/// variants for each supported exchange attribute value. This provides
/// strong typing of exchange metadata.
///
/// Converting between u8 values and variants is handled by trait
/// implementations like `From` and `Into`.
///
pub mod account_type;
pub mod exchange_id;
pub mod security_type;
