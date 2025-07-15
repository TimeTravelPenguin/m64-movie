use binrw::NullString;
use m64_movie::{
    EncodedFixedStrError, MovieError,
    shared::{EncodedFixedStr, FixedStr},
};

#[test]
fn test_fixed_str_new() {
    let input = "Hello, world!";
    let fixed_str: FixedStr<14> = FixedStr::new(input).unwrap();
    assert_eq!(fixed_str.to_string(), input);

    let input = "こんにちは、世界！";
    let fixed_str: FixedStr<28> = FixedStr::new(input).unwrap();
    assert_eq!(fixed_str.to_string(), input);

    let input = "Привет, мир!";
    let fixed_str: FixedStr<22> = FixedStr::new(input).unwrap();
    assert_eq!(fixed_str.to_string(), input);
}

#[test]
fn test_fixed_str_display() {
    let input = "Hello, world!";
    let fixed_str: FixedStr<14> = FixedStr::new(input).unwrap();
    assert_eq!(format!("{}", fixed_str), input);

    let input = "こんにちは、世界！";
    let fixed_str: FixedStr<28> = FixedStr::new(input).unwrap();
    assert_eq!(format!("{}", fixed_str), input);

    let input = "Привет, мир!";
    let fixed_str: FixedStr<22> = FixedStr::new(input).unwrap();
    assert_eq!(format!("{}", fixed_str), input);
}

#[test]
fn test_fixed_str_try_from_str() {
    let input = "Hello, world!";
    let fixed_str: FixedStr<14> = FixedStr::try_from(input).unwrap();
    assert_eq!(fixed_str.to_string(), input);

    let input = "こんにちは、世界！";
    let fixed_str: FixedStr<28> = FixedStr::try_from(input).unwrap();
    assert_eq!(fixed_str.to_string(), input);

    let input = "Привет, мир!";
    let fixed_str: FixedStr<22> = FixedStr::try_from(input).unwrap();
    assert_eq!(fixed_str.to_string(), input);
}

#[test]
fn test_fixed_str_try_from_string() {
    let input = "Hello, world!".to_string();
    let fixed_str: FixedStr<14> = FixedStr::try_from(input).unwrap();
    assert_eq!(fixed_str.to_string(), "Hello, world!");

    let input = "こんにちは、世界！".to_string();
    let fixed_str: FixedStr<28> = FixedStr::try_from(input).unwrap();
    assert_eq!(fixed_str.to_string(), "こんにちは、世界！");

    let input = "Привет, мир!".to_string();
    let fixed_str: FixedStr<22> = FixedStr::try_from(input).unwrap();
    assert_eq!(fixed_str.to_string(), "Привет, мир!");
}

#[test]
fn test_fixed_str_try_from_null_string() {
    let input = NullString::from("Hello, world!");
    let fixed_str: FixedStr<14> = FixedStr::try_from(input).unwrap();
    assert_eq!(fixed_str.to_string(), "Hello, world!");

    let input = NullString::from("こんにちは、世界！");
    let fixed_str: FixedStr<28> = FixedStr::try_from(input).unwrap();
    assert_eq!(fixed_str.to_string(), "こんにちは、世界！");

    let input = NullString::from("Привет, мир!");
    let fixed_str: FixedStr<22> = FixedStr::try_from(input).unwrap();
    assert_eq!(fixed_str.to_string(), "Привет, мир!");
}

#[test]
fn test_null_string_from_fixed_str() {
    let fixed_str: FixedStr<14> = FixedStr::new("Hello, world!").unwrap();
    let null_string: NullString = fixed_str.into();
    assert_eq!(null_string.to_string(), "Hello, world!");

    let fixed_str: FixedStr<28> = FixedStr::new("こんにちは、世界！").unwrap();
    let null_string: NullString = fixed_str.into();
    assert_eq!(null_string.to_string(), "こんにちは、世界！");

    let fixed_str: FixedStr<22> = FixedStr::new("Привет, мир!").unwrap();
    let null_string: NullString = fixed_str.into();
    assert_eq!(null_string.to_string(), "Привет, мир!");
}

// EncodedFixedStr tests

#[test]
fn test_encoded_fixed_str_ascii() {
    let fixed_str: FixedStr<14> = FixedStr::new("Hello, world!").unwrap();
    let encoded = EncodedFixedStr::Ascii(fixed_str);
    match encoded {
        EncodedFixedStr::Ascii(s) => assert_eq!(s.to_string(), "Hello, world!"),
        _ => panic!("Expected EncodedFixedStr::Ascii"),
    }
}

#[test]
fn test_encoded_fixed_str_utf8() {
    let fixed_str: FixedStr<28> = FixedStr::new("こんにちは、世界！").unwrap();
    let encoded = EncodedFixedStr::Utf8(fixed_str);
    match encoded {
        EncodedFixedStr::Utf8(s) => assert_eq!(s.to_string(), "こんにちは、世界！"),
        _ => panic!("Expected EncodedFixedStr::Utf8"),
    }
}

#[test]
fn test_encoded_fixed_str_from_ascii() {
    let input = "Hello, world!";
    let encoded = EncodedFixedStr::<14>::from_ascii(input.as_bytes()).unwrap();
    match encoded {
        EncodedFixedStr::Ascii(s) => assert_eq!(s.to_string(), input),
        _ => panic!("Expected EncodedFixedStr::Ascii"),
    }
}

#[test]
fn test_encoded_fixed_str_from_utf8() {
    let input = "こんにちは、世界！";
    let encoded = EncodedFixedStr::<28>::from_utf8(input.as_bytes()).unwrap();
    match encoded {
        EncodedFixedStr::Utf8(s) => assert_eq!(s.to_string(), input),
        _ => panic!("Expected EncodedFixedStr::Utf8"),
    }
}

#[test]
fn test_encoded_fixed_str_error_invalid_ascii() {
    let input = "Hello, 世界!";
    let result = EncodedFixedStr::<15>::from_ascii(input.as_bytes());

    assert!(result.is_err());

    if let Err(MovieError::StringError(EncodedFixedStrError::InvalidAscii(s))) = result {
        assert_eq!(s, "Hello, 世界!");
    } else {
        panic!("Expected InvalidAscii error");
    }
}
