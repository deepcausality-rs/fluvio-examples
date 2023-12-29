use common::prelude::InitError;

#[test]
fn test_new_init_error() {
    let error = InitError("test error".to_string());
    assert_eq!("InitError: test error", error.to_string());
}

#[test]
fn test_debug() {
    let error = InitError("test error".to_string());
    assert_eq!("InitError(\"test error\")", format!("{:?}", error));
}

#[test]
fn test_display() {
    let error = InitError("test error".to_string());
    assert_eq!("InitError: test error", format!("{}", error));
}
