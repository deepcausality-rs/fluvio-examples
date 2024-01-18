use sbe_messages::prelude::{FirstOHLCVBar, MessageType};

#[test]
fn test_new() {
    let bar = FirstOHLCVBar::new(42);

    assert_eq!(bar.message_type(), MessageType::FirstOHLCVBar);
}
#[test]
fn test_message_type() {
    let bar = FirstOHLCVBar::new(42);

    assert_eq!(bar.message_type(), MessageType::FirstOHLCVBar);
}

#[test]
fn test_encode() {
    let bar = FirstOHLCVBar::new(42);

    assert_eq!(bar.message_type(), MessageType::FirstOHLCVBar);

    let enc = bar.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 12);

    let expected: Vec<u8> = vec![4, 0, 205, 0, 1, 0, 1, 0, 205, 0, 42, 0];
    let actual = buffer;

    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let encoded: Vec<u8> = vec![4, 0, 205, 0, 1, 0, 1, 0, 205, 0, 42, 0];
    let buffer = encoded.as_slice();

    let message = FirstOHLCVBar::from(buffer);
    assert_eq!(message.message_type(), MessageType::FirstOHLCVBar);
}

#[test]
fn test_display() {
    let bar = FirstOHLCVBar::new(42);

    let expected = "FirstOHLCVBar { message_type: FirstOHLCVBar, symbol_id: 42 }";
    let actual = format!("{}", bar);

    assert_eq!(expected, actual);
}
