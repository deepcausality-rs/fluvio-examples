use common::prelude::MessageProcessingError;

#[test]
fn test_new() {
    let error = MessageProcessingError("test error".to_string());
    assert_eq!("MessageProcessingError: test error", error.to_string());
}

#[test]
fn test_debug() {
    let error = MessageProcessingError("test error".to_string());
    assert_eq!(
        "MessageProcessingError(\"test error\")",
        format!("{:?}", error)
    );
}

#[test]
fn test_display() {
    let error = MessageProcessingError("test error".to_string());
    assert_eq!("MessageProcessingError: test error", format!("{}", error));
}
