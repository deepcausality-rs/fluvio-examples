use sbe_messages::prelude::{ClientErrorMessage, ClientErrorType, MessageType};

#[test]
fn test_new() {
    let client_id = 1;
    let client_error_type = ClientErrorType::ClientAlreadyLoggedIn;
    let message = ClientErrorMessage::new(client_id, client_error_type);

    assert_eq!(message.message_type(), MessageType::ClientError);
    assert_eq!(message.client_id(), client_id);
    assert_eq!(message.client_error_type(), client_error_type);
}

#[test]
fn test_encode() {
    let client_id = 1;
    let client_error_type = ClientErrorType::ClientAlreadyLoggedIn;
    let message = ClientErrorMessage::new(client_id, client_error_type);

    assert_eq!(message.message_type(), MessageType::ClientError);
    assert_eq!(message.client_id(), client_id);
    assert_eq!(message.client_error_type(), client_error_type);

    let enc = message.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 13);

    let expected: Vec<u8> = vec![5, 0, 33, 3, 1, 0, 1, 0, 33, 3, 1, 0, 1];
    let actual = buffer;
    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let encoded: Vec<u8> = vec![5, 0, 33, 3, 1, 0, 1, 0, 33, 3, 1, 0, 1];
    let buffer = encoded.as_slice();

    let message = ClientErrorMessage::from(buffer);

    let client_id = 1;
    let client_error_type = ClientErrorType::ClientAlreadyLoggedIn;

    assert_eq!(message.message_type(), MessageType::ClientError);
    assert_eq!(message.client_id(), client_id);
    assert_eq!(message.client_error_type(), client_error_type);
}

#[test]
fn test_display() {
    let client_id = 1;
    let client_error_type = ClientErrorType::ClientAlreadyLoggedIn;
    let message = ClientErrorMessage::new(client_id, client_error_type);

    let expected = "ClientErrorMessage { message_type: ClientError, client_id: 1, client_error_type: ClientAlreadyLoggedIn }";
    let actual = format!("{}", message);
    assert_eq!(expected, actual);
}
