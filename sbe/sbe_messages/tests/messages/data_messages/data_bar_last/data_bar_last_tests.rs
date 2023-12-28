use sbe_messages::prelude::{LastDataBar, MessageType};

#[test]
fn test_new() {
    let bar = LastDataBar::new();

    assert_eq!(bar.message_type(), MessageType::LastDataBar);
}

#[test]
fn test_message_type() {
    let bar = LastDataBar::new();

    assert_eq!(bar.message_type(), MessageType::LastDataBar);
}

#[test]
fn test_encode() {
    let message = LastDataBar::new();
    assert_eq!(message.message_type(), MessageType::LastDataBar);

    let enc = message.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 9);

    let expected: Vec<u8> = vec![1, 0, 7, 0, 1, 0, 1, 0, 7];
    let actual = buffer;

    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let encoded: Vec<u8> = vec![1, 0, 7, 0, 1, 0, 1, 0, 7];
    let buffer = encoded.as_slice();

    let message = LastDataBar::from(buffer);
    assert_eq!(message.message_type(), MessageType::LastDataBar);
}

#[test]
fn test_display() {
    let bar = LastDataBar::new();

    let expected = "LastDataBar { message_type: LastDataBar }";
    let actual = format!("{}", bar);

    assert_eq!(expected, actual);
}
