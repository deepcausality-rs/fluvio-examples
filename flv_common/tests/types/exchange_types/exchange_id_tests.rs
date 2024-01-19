use common::prelude::ExchangeID;

#[test]
fn test_from_valid_values() {
    assert_eq!(ExchangeID::from(0x0), ExchangeID::NullVal);
    assert_eq!(ExchangeID::from(0x1), ExchangeID::Kraken);
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
fn test_kraken() {
    let exchange_id = ExchangeID::Kraken;
    assert_eq!(format!("{}", exchange_id), "Kraken");
}
