use chrono::{Datelike, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use common::prelude::TradeBar;

#[test]
fn test_new() {
    let bar = TradeBar::new(
        Utc::now(),
        Decimal::from(100),
        Decimal::from(50)
    );

    assert_eq!(bar.price(), Decimal::from(100));
    assert_eq!(bar.volume(), Decimal::from(50));
}

#[test]
fn test_default() {
    let bar = TradeBar::default();

    assert_eq!(bar.date_time().day(), Utc::now().day());
    assert_eq!(bar.price(), Decimal::zero());
    assert_eq!(bar.volume(), Decimal::zero());
}

#[test]
fn test_price() {
    let bar = TradeBar::new(Utc::now(), Decimal::from(100), Decimal::zero());
    assert_eq!(bar.price(), Decimal::from(100));
}

#[test]
fn test_volume() {
    let bar = TradeBar::new(Utc::now(), Decimal::zero(), Decimal::from(50));
    assert_eq!(bar.volume(), Decimal::from(50));
}

#[test]
fn test_display() {
    let bar = TradeBar::new(
        Utc::now(),
        Decimal::from(100),
        Decimal::from(50)
    );

    let expected = format!("timestamp: {}: price={}, volume={}",
                           bar.date_time(), bar.price(), bar.volume());

    assert_eq!(expected, format!("{}", bar));
}