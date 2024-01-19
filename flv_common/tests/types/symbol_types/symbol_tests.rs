use common::prelude::{ExchangeID, Symbol};
use rust_decimal::Decimal;
use std::str::FromStr;

fn get_new_symbol() -> Symbol {
    Symbol::new(
        "BTCUSD".to_string(),
        "BTC-USD".to_string(),
        ExchangeID::Kraken,
        "BTC".to_string(),
        "USD".to_string(),
        Decimal::from_str("0.000001").unwrap(),
        Decimal::from_str("0.001").unwrap(),
    )
}

#[test]
fn test_symbol_id_global() {
    let symbol = get_new_symbol();

    assert_eq!(symbol.symbol_id_global(), "BTCUSD");
}

#[test]
fn test_symbol_id_exchange() {
    let symbol = get_new_symbol();

    assert_eq!(symbol.symbol_id_exchange(), "BTC-USD");
}

#[test]
fn test_exchange_id() {
    let symbol = get_new_symbol();

    assert_eq!(symbol.exchange_id(), &ExchangeID::Kraken);
}

#[test]
fn test_asset_base_exchange() {
    let symbol = get_new_symbol();

    assert_eq!(symbol.asset_base_exchange(), "BTC");
}

#[test]
fn test_asset_quote_exchange() {
    let symbol = get_new_symbol();

    assert_eq!(symbol.asset_quote_exchange(), "USD");
}

#[test]
fn test_price_precision() {
    let symbol = get_new_symbol();

    assert_eq!(
        symbol.price_precision(),
        Decimal::from_str("0.000001").unwrap()
    );
}

#[test]
fn test_size_precision() {
    let symbol = get_new_symbol();

    assert_eq!(symbol.size_precision(), Decimal::from_str("0.001").unwrap());
}
