use common::prelude::MessageClientConfigError;

#[test]
fn test_new() {
    let err = MessageClientConfigError("error message".to_string());
    assert_eq!(
        format!("{}", err),
        "MessageClientConfigError: error message"
    );
}

#[test]
fn test_debug() {
    let err = MessageClientConfigError("error message".to_string());

    assert_eq!(
        format!("{}", err),
        "MessageClientConfigError: error message"
    );
    assert_eq!(
        format!("{:?}", err),
        "MessageClientConfigError(\"error message\")"
    );
}

#[test]
fn test_display() {
    let err = MessageClientConfigError("error message".to_string());
    assert_eq!(
        format!("{}", err),
        "MessageClientConfigError: error message"
    );
}
