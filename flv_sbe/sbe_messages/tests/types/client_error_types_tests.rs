use sbe_messages::prelude::ClientErrorType;

#[test]
fn test_from_u8() {
    assert_eq!(
        ClientErrorType::from(0),
        ClientErrorType::UnknownClientError
    );
    assert_eq!(
        ClientErrorType::from(1),
        ClientErrorType::ClientAlreadyLoggedIn
    );
    assert_eq!(ClientErrorType::from(2), ClientErrorType::ClientLogInError);
    assert_eq!(ClientErrorType::from(3), ClientErrorType::ClientNotLoggedIn);
    assert_eq!(ClientErrorType::from(4), ClientErrorType::ClientLogOutError);
    assert_eq!(
        ClientErrorType::from(5),
        ClientErrorType::UnknownClientError
    );
}

#[test]
fn test_display() {
    let error = ClientErrorType::UnknownClientError;
    let expected = "UnknownClientError";
    let actual = format!("{}", error);
    assert_eq!(expected, actual);

    let error = ClientErrorType::ClientAlreadyLoggedIn;
    let expected = "ClientAlreadyLoggedIn";
    let actual = format!("{}", error);
    assert_eq!(expected, actual);
}
