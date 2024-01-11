use sbe_messages::prelude::{FirstTradeBar, MessageType};

#[test]
fn test_new() {
    let bar = FirstTradeBar::new(42);

    assert_eq!(bar.message_type(), MessageType::FirstTradeBar);
}

#[test]
fn test_message_type() {
    let bar = FirstTradeBar::new(42);
    assert_eq!(bar.message_type(), MessageType::FirstTradeBar);
}

#[test]
fn test_symbol_id() {
    let bar = FirstTradeBar::new(123);
    assert_eq!(bar.symbol_id(), 123);
}

#[test]
fn test_encode() {
    let bar = FirstTradeBar::new(123);
    assert_eq!(bar.message_type(), MessageType::FirstTradeBar);
    assert_eq!(bar.symbol_id(), 123);

    let enc = bar.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 12);

    let expected: Vec<u8> = vec![4, 0, 208, 0, 1, 0, 1, 0, 208, 0, 123, 0];
    let actual = buffer;

    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let encoded: Vec<u8> = vec![4, 0, 208, 0, 1, 0, 1, 0, 208, 0, 123, 0];
    let buffer = encoded.as_slice();

    let message = FirstTradeBar::from(buffer);
    assert_eq!(message.message_type(), MessageType::FirstTradeBar);
}

#[test]
fn test_display() {
    let bar = FirstTradeBar::new(123);

    let expected = "FirstTradeBar { message_type: FirstTradeBar, symbol_id: 123 }";

    let actual = format!("{}", bar);

    assert_eq!(actual, expected);
}
