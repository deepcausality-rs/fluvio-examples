use lib_inference::prelude::BarRange;
use rust_decimal::Decimal;

#[test]
fn test_bar_range_new() {
    let high = Decimal::from(100);
    let close = Decimal::from(90);
    let close_above_open = false;
    let close_below_open = true;

    let bar_range = BarRange::new(high, close, close_above_open, close_below_open);

    assert_eq!(bar_range.high(), high);
    assert_eq!(bar_range.close(), close);
    assert_eq!(bar_range.close_above_open(), close_above_open);
    assert_eq!(bar_range.close_below_open(), close_below_open);
}

#[test]
fn test_bar_range_accessors() {
    let high = Decimal::from(100);
    let close = Decimal::from(90);
    let close_above_open = false;
    let close_below_open = true;

    let bar_range = BarRange::new(high, close, close_above_open, close_below_open);

    assert_eq!(bar_range.high(), high);
    assert_eq!(bar_range.close(), close);
    assert_eq!(bar_range.close_above_open(), close_above_open);
    assert_eq!(bar_range.close_below_open(), close_below_open);
}

#[test]
fn test_bar_range_display() {
    let high = Decimal::from(100);
    let close = Decimal::from(90);
    let close_above_open = false;
    let close_below_open = true;

    let bar_range = BarRange::new(high, close, close_above_open, close_below_open);

    let expected =
        "BarRange { high: 100, close: 90, close_above_open: false, close_below_open: true }";
    let actual = format!("{}", bar_range);

    assert_eq!(expected, actual);
}
