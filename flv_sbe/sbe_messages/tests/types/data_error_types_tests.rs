use sbe_messages::prelude::DataErrorType;

#[test]
fn test_from_u8() {
    assert_eq!(DataErrorType::from(0), DataErrorType::UnknownDataError);
    assert_eq!(DataErrorType::from(1), DataErrorType::DataNotKnownError);
    assert_eq!(DataErrorType::from(2), DataErrorType::DataUnavailableError);
    assert_eq!(DataErrorType::from(3), DataErrorType::DataEncodingError);
    assert_eq!(DataErrorType::from(4), DataErrorType::UnknownDataError);
}

#[test]
fn test_display() {
    let error = DataErrorType::UnknownDataError;
    let expected = "UnknownDataError";
    let actual = format!("{}", error);
    assert_eq!(expected, actual);

    let error = DataErrorType::DataNotKnownError;
    let expected = "DataNotKnownError";
    let actual = format!("{}", error);
    assert_eq!(expected, actual);
}
