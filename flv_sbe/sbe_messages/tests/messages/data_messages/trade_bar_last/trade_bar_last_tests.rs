use sbe_messages::prelude::{LastTradeBar, MessageType};

#[test]
fn test_new() {
    let bar = LastTradeBar::new(23);
    assert_eq!(bar.message_type(), MessageType::LastTradeBar);
}

#[test]
fn test_message_type() {
    let bar = LastTradeBar::new(23);
    assert_eq!(bar.message_type(), MessageType::LastTradeBar);
}

#[test]
fn test_symbol_id() {
    let bar = LastTradeBar::new(123);
    assert_eq!(bar.symbol_id(), 123);
}

#[test]
fn test_encode() {
    let bar = LastTradeBar::new(23);
    assert_eq!(bar.message_type(), MessageType::LastTradeBar);

    let enc = bar.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 12);

    let expected: Vec<u8> = vec![4, 0, 209, 0, 1, 0, 1, 0, 209, 0, 23, 0];
    let actual = buffer;

    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let encoded: Vec<u8> = vec![4, 0, 209, 0, 1, 0, 1, 0, 209, 0, 23, 0];
    let buffer = encoded.as_slice();

    let message = LastTradeBar::from(buffer);
    assert_eq!(message.message_type(), MessageType::LastTradeBar);
    assert_eq!(message.symbol_id(), 23);
}

#[test]
fn test_fmt() {
    let bar = LastTradeBar::new(123);

    let expected = "LastTradeBar { message_type: LastTradeBar, symbol_id: 123 }";
    let actual = format!("{}", bar);

    assert_eq!(actual, expected);
}
