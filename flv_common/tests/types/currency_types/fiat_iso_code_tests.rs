use common::prelude::FiatIsoCode;

#[test]
fn test_new() {
    let code = FiatIsoCode::new("USD".to_string(), ['U', 'S', 'D'], 840);

    assert_eq!(code.currency(), "USD");
    assert_eq!(code.alphabetic_code(), "USD");
    assert_eq!(code.iso_code(), 840);
}

#[test]
fn test_currency() {
    let code = FiatIsoCode::new("EUR".to_string(), ['E', 'U', 'R'], 978);

    assert_eq!(code.currency(), "EUR");
}

#[test]
fn test_alphabetic_code() {
    let code = FiatIsoCode::new("JPY".to_string(), ['J', 'P', 'Y'], 392);

    assert_eq!(code.alphabetic_code(), "JPY");
}

#[test]
fn test_iso_code() {
    let code = FiatIsoCode::new("CHF".to_string(), ['C', 'H', 'F'], 756);

    assert_eq!(code.iso_code(), 756);
}

#[test]
fn test_display() {
    let code = FiatIsoCode::new("USD".to_string(), ['U', 'S', 'D'], 840);

    let expected = "USD (USD, 840)";
    let actual = format!("{}", code);

    assert_eq!(actual, expected);
}
