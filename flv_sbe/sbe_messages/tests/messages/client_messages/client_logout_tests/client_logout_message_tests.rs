use sbe_messages::prelude::{ClientLogoutMessage, MessageType};

#[test]
fn test_new() {
    let message = ClientLogoutMessage::new(123);

    assert_eq!(message.message_type(), &MessageType::ClientLogout);
    assert_eq!(message.client_id(), 123);
}

#[test]
fn test_encode() {
    let client_id = 22;
    let message = ClientLogoutMessage::new(client_id);

    assert_eq!(message.message_type(), &MessageType::ClientLogout);
    assert_eq!(message.client_id(), client_id);

    let enc = message.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 11);

    let expected: Vec<u8> = vec![3, 0, 2, 0, 1, 0, 1, 0, 2, 22, 0];
    let actual = buffer;
    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let encoded: Vec<u8> = vec![3, 0, 2, 0, 1, 0, 1, 0, 2, 22, 0];
    let buffer = encoded.as_slice();

    let message = ClientLogoutMessage::from(buffer);
    assert_eq!(message.message_type(), &MessageType::ClientLogout);
    assert_eq!(message.client_id(), 22);
}

#[test]
fn test_message_type() {
    let client_id = 22;
    let message = ClientLogoutMessage::new(client_id);

    assert_eq!(message.message_type(), &MessageType::ClientLogout);
}

#[test]
fn test_message_client_id() {
    let client_id = 22;
    let message = ClientLogoutMessage::new(client_id);

    assert_eq!(message.client_id(), client_id);
}

#[test]
fn test_display() {
    let client_id = 123;
    let message = ClientLogoutMessage::new(client_id);

    let expected = "ClientLogoutMessage { client_id: 123 }";

    assert_eq!(format!("{}", message), expected);
}
