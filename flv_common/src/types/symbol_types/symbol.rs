use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::prelude::ExchangeID;

/// Symbol struct definition.
///
/// Holds symbol metadata and specifications.
///
/// # Fields
///
/// `symbol_id_global` - Global unique symbol ID
/// `symbol_id_exchange` - Exchange specific symbol ID
/// `exchange_id` - ExchangeID enum
/// `asset_base_exchange` - Base asset symbol on exchange
/// `asset_quote_exchange` - Quote asset symbol on exchange
/// `price_precision` - Price decimal precision
/// `size_precision` - Size/volume decimal precision
///
/// # Implements
///
/// `Serialize`, `Deserialize` - Serde serialization
/// `Debug`, `Default`, `Clone`, `Eq`, `PartialEq` - Rust defaults
///
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Symbol {
    symbol_id_global: String,
    symbol_id_exchange: String,
    exchange_id: ExchangeID,
    asset_base_exchange: String,
    asset_quote_exchange: String,
    price_precision: Decimal,
    size_precision: Decimal,
}

impl Symbol {
    /// Creates a new Symbol instance.
    ///
    /// # Arguments
    ///
    /// * `symbol_id_global` - Global unique symbol ID
    /// * `symbol_id_exchange` - Exchange specific symbol ID
    /// * `exchange_id` - ExchangeID enum
    /// * `asset_base_exchange` - Base asset symbol on exchange
    /// * `asset_quote_exchange` - Quote asset symbol on exchange
    /// * `price_precision` - Price decimal precision
    /// * `size_precision` - Size/volume decimal precision
    ///
    /// # Returns
    ///
    /// Symbol instance
    ///
    pub fn new(
        symbol_id_global: String,
        symbol_id_exchange: String,
        exchange_id: ExchangeID,
        asset_base_exchange: String,
        asset_quote_exchange: String,
        price_precision: Decimal,
        size_precision: Decimal,
    ) -> Self {
        Self {
            symbol_id_global,
            symbol_id_exchange,
            exchange_id,
            asset_base_exchange,
            asset_quote_exchange,
            price_precision,
            size_precision,
        }
    }
}

impl Symbol {
    pub fn symbol_id_global(&self) -> &str {
        &self.symbol_id_global
    }
    pub fn symbol_id_exchange(&self) -> &str {
        &self.symbol_id_exchange
    }
    pub fn exchange_id(&self) -> &ExchangeID {
        &self.exchange_id
    }
    pub fn asset_base_exchange(&self) -> &str {
        &self.asset_base_exchange
    }
    pub fn asset_quote_exchange(&self) -> &str {
        &self.asset_quote_exchange
    }
    pub fn price_precision(&self) -> Decimal {
        self.price_precision
    }
    pub fn size_precision(&self) -> Decimal {
        self.size_precision
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Symbol[global_id: {}, exchange_id: {}, base: {}, quote: {}, price_precision: {}, size_precision: {}]",
            self.symbol_id_global,
            self.symbol_id_exchange,
            self.asset_base_exchange,
            self.asset_quote_exchange,
            self.price_precision,
            self.size_precision
        )
    }
}
