use chrono::{DateTime, Utc};
use common::prelude::TradeBar;
use rust_decimal::Decimal;
use sbe_messages::prelude::SbeTradeBar;

// Default uses utc::now() for date_time, which is not deterministic,
// and that would cause the encode test to fail. Therefore we use a fixed date.
fn get_trade_bar() -> TradeBar {
    let date_str = "2022-04-12T22:10:57+02:00";
    // convert the string into DateTime<FixedOffset>
    let datetime_fixed = DateTime::parse_from_rfc3339(date_str).unwrap();
    // convert the string into DateTime<Utc> or other timezone
    let date_time = datetime_fixed.with_timezone(&Utc);
    let price = Decimal::from(100);
    let volume = Decimal::from(100);
    //
    TradeBar::new(date_time, price, volume)
}

#[test]
fn test_new() {
    let trade_bar = SbeTradeBar::new();
    assert_eq!(trade_bar, SbeTradeBar::default());
}

#[test]
fn test_encode_data_bar_message() {
    let bar = get_trade_bar();

    let result = SbeTradeBar::encode_data_bar_message(bar);

    assert!(result.is_ok()); // Assert encode passes

    let (size, encoded) = result.unwrap();
    assert_eq!(size, 28); // Assert encoded message size matches expected
    assert!(!encoded.is_empty()); // Assert non-empty encoded message

    let actual = encoded;
    let expected: Vec<u8> = vec![20, 0, 207, 0, 1, 0, 1, 0, 207, 0, 0, 0, 64, 22, 164, 168, 122, 220, 5, 0, 0, 0, 200, 66, 0, 0, 200, 66];

    assert_eq!(expected, actual);
}

#[test]
fn test_decode_trade_bar_message() {
    let encoded: Vec<u8> =  vec![20, 0, 207, 0, 1, 0, 1, 0, 207, 0, 0, 0, 64, 22, 164, 168, 122, 220, 5, 0, 0, 0, 200, 66, 0, 0, 200, 66];

    let message = SbeTradeBar::decode_trade_bar_message(&encoded).unwrap();

    assert_eq!(message.price(), Decimal::from(100));
    assert_eq!(message.volume(), Decimal::from(100));
}
