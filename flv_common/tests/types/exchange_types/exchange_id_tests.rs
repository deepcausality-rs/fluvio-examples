use common::prelude::ExchangeID;

#[test]
fn test_from_valid_values() {
    assert_eq!(ExchangeID::from(0x0), ExchangeID::NullVal);
    assert_eq!(ExchangeID::from(0x1), ExchangeID::BinanceSpot);
    assert_eq!(ExchangeID::from(0x2), ExchangeID::COINBASE);
    assert_eq!(ExchangeID::from(0x3), ExchangeID::VEX);
}

#[test]
fn test_from_invalid_values() {
    assert_eq!(ExchangeID::from(0x0), ExchangeID::NullVal);
}

#[test]
fn test_null_val() {
    let exchange_id = ExchangeID::NullVal;
    assert_eq!(format!("{}", exchange_id), "NullVal");
}

#[test]
fn test_binance() {
    let exchange_id = ExchangeID::BinanceSpot;
    assert_eq!(format!("{}", exchange_id), "BinanceSpot");
}

#[test]
fn test_vex() {
    let exchange_id = ExchangeID::VEX;
    assert_eq!(format!("{}", exchange_id), "VEX");
}
