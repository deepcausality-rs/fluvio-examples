use sbe_messages::prelude::{DataErrorMessage, DataErrorType, MessageType};

#[test]
fn test_new() {
    let client_id = 1;
    let error_type = DataErrorType::DataNotKnownError;
    let message = DataErrorMessage::new(client_id, error_type);

    assert_eq!(message.message_type(), MessageType::DataError);
    assert_eq!(message.client_id(), client_id);
    assert_eq!(message.data_error_type(), error_type);
}

#[test]
fn test_encode() {
    let client_id = 1;
    let error_type = DataErrorType::DataNotKnownError;
    let message = DataErrorMessage::new(client_id, error_type);

    assert_eq!(message.message_type(), MessageType::DataError);
    assert_eq!(message.client_id(), client_id);
    assert_eq!(message.data_error_type(), error_type);

    let enc = message.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 13);

    let expected: Vec<u8> = vec![5, 0, 34, 3, 1, 0, 1, 0, 34, 3, 1, 0, 1];
    let actual = buffer;
    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let encoded: Vec<u8> = vec![5, 0, 34, 3, 1, 0, 1, 0, 34, 3, 1, 0, 1];
    let buffer = encoded.as_slice();
    let message = DataErrorMessage::from(buffer);

    let client_id = 1;
    let error_type = DataErrorType::DataNotKnownError;
    assert_eq!(message.message_type(), MessageType::DataError);
    assert_eq!(message.client_id(), client_id);
    assert_eq!(message.data_error_type(), error_type);
}

#[test]
fn test_display() {
    let client_id = 1;
    let error_type = DataErrorType::DataNotKnownError;
    let message = DataErrorMessage::new(client_id, error_type);

    let expected = "DataErrorMessage { message_type: DataError, client_id: 1, data_error_type: DataNotKnownError }";
    let actual = format!("{}", message);

    assert_eq!(expected, actual);
}
