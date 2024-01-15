use chrono::{Datelike, Utc};
use common::prelude::TradeBar;
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;

#[test]
fn test_new() {
    let bar = TradeBar::new(1, Utc::now(), Decimal::from(100), Decimal::from(50));

    assert_eq!(bar.symbol_id(), 1);
    assert_eq!(bar.price(), Decimal::from(100));
    assert_eq!(bar.volume(), Decimal::from(50));
}

#[test]
fn test_default() {
    let bar = TradeBar::default();

    assert_eq!(bar.symbol_id(), 1);
    assert_eq!(bar.date_time().day(), Utc::now().day());
    assert_eq!(bar.price(), Decimal::zero());
    assert_eq!(bar.volume(), Decimal::zero());
}

#[test]
fn test_symbol_id() {
    let bar = TradeBar::new(1, Utc::now(), Decimal::from(100), Decimal::from(50));

    assert_eq!(bar.symbol_id(), 1);
}

#[test]
fn test_price() {
    let bar = TradeBar::new(1, Utc::now(), Decimal::from(100), Decimal::from(50));
    assert_eq!(bar.price(), Decimal::from(100));
}

#[test]
fn test_volume() {
    let bar = TradeBar::new(1, Utc::now(), Decimal::from(100), Decimal::from(50));
    assert_eq!(bar.volume(), Decimal::from(50));
}

#[test]
fn test_display() {
    let bar = TradeBar::new(1, Utc::now(), Decimal::from(100), Decimal::from(50));

    let expected = format!(
        "symbol_id: {}, timestamp: {}: price={}, volume={}",
        bar.symbol_id(),
        bar.date_time(),
        bar.price(),
        bar.volume()
    );

    assert_eq!(expected, format!("{}", bar));
}
