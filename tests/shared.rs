use binrw::NullString;
use m64_movie::{EncodedFixedStrError, MovieError, shared::EncodedFixedStr};

#[test]
fn test_encoded_fixed_str_ascii() {
    let encoded = EncodedFixedStr::<14, _>::from_ascii_str("Hello, world!").unwrap();
    assert_eq!(encoded.to_string(), "Hello, world!");
}

#[test]
fn test_encoded_fixed_str_utf8() {
    let encoded = EncodedFixedStr::<28, _>::from_utf8_str("こんにちは、世界！").unwrap();
    assert_eq!(encoded.to_string(), "こんにちは、世界！");
}

#[test]
fn test_encoded_fixed_str_error_invalid_ascii() {
    let input = "Hello, 世界!";
    let result = EncodedFixedStr::<15, _>::from_ascii(input.as_bytes());

    assert!(result.is_err());

    if let Err(MovieError::FixedStrError(EncodedFixedStrError::InvalidAscii(s))) = result {
        assert_eq!(s, "Hello, 世界!");
    } else {
        panic!("Expected InvalidAscii error");
    }
}

#[test]
fn test_encoded_fixed_str_ascii_into_null_string() {
    let encoded = EncodedFixedStr::<14, _>::from_ascii_str("Hello, world!").unwrap();
    let s: NullString = encoded.into();

    assert_eq!(s.to_string(), "Hello, world!");
}

#[test]
fn test_encoded_fixed_str_utf8_into_null_string() {
    let encoded = EncodedFixedStr::<28, _>::from_utf8_str("こんにちは、世界！").unwrap();
    let s: NullString = encoded.into();

    assert_eq!(s.to_string(), "こんにちは、世界！");
}
