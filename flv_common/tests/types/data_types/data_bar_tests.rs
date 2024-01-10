use chrono::Utc;
use common::prelude::{DataBar};
use rust_decimal::Decimal;

#[test]
fn test_new() {
    let date_time = Utc::now();
    let open = Decimal::default();
    let high = Decimal::default();
    let low = Decimal::default();
    let close = Decimal::default();
    let volume = Decimal::default();

    let data_bar = DataBar::new(date_time, open, high, low, close, volume);

    assert_eq!(data_bar.date_time(), date_time);
    assert_eq!(data_bar.open(), open);
    assert_eq!(data_bar.high(), high);
    assert_eq!(data_bar.low(), low);
    assert_eq!(data_bar.close(), close);
    assert_eq!(data_bar.volume(), volume);
}

#[test]
fn test_default() {
    let open = Decimal::default();
    let high = Decimal::default();
    let low = Decimal::default();
    let close = Decimal::default();
    let volume = Decimal::default();

    let default_bar = DataBar::default();

    assert_eq!(default_bar.open(), open);
    assert_eq!(default_bar.high(), high);
    assert_eq!(default_bar.low(), low);
    assert_eq!(default_bar.close(), close);
    assert_eq!(default_bar.volume(), volume);
}
