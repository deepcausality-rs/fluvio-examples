use chrono::Utc;
use common::prelude::OHLCVBar;
use rust_decimal::Decimal;

#[test]
fn test_new() {
    let symbol_id = 1;
    let date_time = Utc::now();
    let open = Decimal::default();
    let high = Decimal::default();
    let low = Decimal::default();
    let close = Decimal::default();
    let volume = Decimal::default();

    let data_bar = OHLCVBar::new(symbol_id, date_time, open, high, low, close, volume);

    assert_eq!(data_bar.symbol_id(), symbol_id);
    assert_eq!(data_bar.date_time(), date_time);
    assert_eq!(data_bar.open(), open);
    assert_eq!(data_bar.high(), high);
    assert_eq!(data_bar.low(), low);
    assert_eq!(data_bar.close(), close);
    assert_eq!(data_bar.volume(), volume);
}

#[test]
fn test_default() {
    let symbol_id = 1;
    let open = Decimal::default();
    let high = Decimal::default();
    let low = Decimal::default();
    let close = Decimal::default();
    let volume = Decimal::default();

    let default_bar = OHLCVBar::default();

    assert_eq!(default_bar.symbol_id(), symbol_id);
    assert_eq!(default_bar.open(), open);
    assert_eq!(default_bar.high(), high);
    assert_eq!(default_bar.low(), low);
    assert_eq!(default_bar.close(), close);
    assert_eq!(default_bar.volume(), volume);
}
