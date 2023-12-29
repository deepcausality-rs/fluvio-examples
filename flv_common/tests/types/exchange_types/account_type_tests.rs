use common::prelude::AccountType;

#[test]
fn test_unknown_account_type_display() {
    let account_type = AccountType::NullVal;
    assert_eq!(account_type.to_string(), "NullVal");
}

#[test]
fn test_spot_account_type_display() {
    let account_type = AccountType::Spot;
    assert_eq!(account_type.to_string(), "Spot");
}

#[test]
fn test_margin_account_type_display() {
    let account_type = AccountType::Margin;
    assert_eq!(account_type.to_string(), "Margin");
}

#[test]
fn test_future_account_type_display() {
    let account_type = AccountType::Future;
    assert_eq!(account_type.to_string(), "Future");
}
