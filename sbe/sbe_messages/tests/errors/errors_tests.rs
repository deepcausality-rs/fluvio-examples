use sbe_messages::prelude::{SbeDecodeError, SbeEncodeError};

#[test]
fn test_sbe_encode_error_display() {
    let error = SbeEncodeError("test error".to_string());
    assert_eq!(error.to_string(), "SbeEncodeError: test error");
}

#[test]
fn test_sbe_decode_error_display() {
    let error = SbeDecodeError("test error".to_string());
    assert_eq!(error.to_string(), "SbeDecodeError: test error");
}
