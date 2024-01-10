use sbe_messages::prelude::{LastDataBar, MessageType};

#[test]
fn test_new() {
    let bar = LastDataBar::new(42);

    assert_eq!(bar.message_type(), MessageType::LastDataBar);
}

#[test]
fn test_message_type() {
    let bar = LastDataBar::new(42);

    assert_eq!(bar.message_type(), MessageType::LastDataBar);
}

#[test]
fn test_encode() {
    let message = LastDataBar::new(42);
    assert_eq!(message.message_type(), MessageType::LastDataBar);

    let enc = message.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 12);

    let expected: Vec<u8> = vec![4, 0, 206, 0, 1, 0, 1, 0, 206, 0, 42, 0];
    let actual = buffer;

    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let encoded: Vec<u8> = vec![4, 0, 206, 0, 1, 0, 1, 0, 206, 0, 42, 0];
    let buffer = encoded.as_slice();

    let message = LastDataBar::from(buffer);
    assert_eq!(message.message_type(), MessageType::LastDataBar);
}

#[test]
fn test_display() {
    let bar = LastDataBar::new(42);

    let expected = "LastDataBar { message_type: LastDataBar, symbol_id: 42 }";
    let actual = format!("{}", bar);

    assert_eq!(expected, actual);
}
