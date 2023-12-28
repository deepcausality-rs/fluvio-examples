use sbe_messages::utils::sbe_encode_utils;

#[test]
fn test_encode_string() {
    let input = "Hello, world!";
    let expected: [u8; 13] = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33];
    let res = sbe_encode_utils::encode_string::<13>(input);
    assert!(res.is_ok());
    let output = res.unwrap();
    assert_eq!(expected, output);
}

#[test]
fn test_encode_string_empty_is_error() {
    let input = "";
    let res = sbe_encode_utils::encode_string::<1>(input);
    assert!(res.is_err());
}

#[test]
fn test_encode_string_longer_than_array_is_error() {
    let input = "This is a longer string than the array can hold.";
    let res = sbe_encode_utils::encode_string::<8>(input);
    assert!(res.is_err())
}

#[test]
fn test_encode_string_single_character() {
    let input = "a";
    let expected: [u8; 1] = [97];
    let res = sbe_encode_utils::encode_string::<1>(input);
    assert!(res.is_ok());
    let output = res.unwrap();
    assert_eq!(expected, output);
}

#[test]
fn test_encode_string_whitespace() {
    let input = "    ";
    let expected: [u8; 4] = [32, 32, 32, 32];
    let res = sbe_encode_utils::encode_string::<4>(input);
    assert!(res.is_ok());
    let output = res.unwrap();
    assert_eq!(expected, output);
}
