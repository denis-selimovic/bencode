use bencode::protocol::{decode, encode};
use bencode::types::Type;


#[test]
fn test_decode_empty_string() {
    let t = Type::ByteString("".to_string());
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::ByteString(s) => assert_eq!(s, ""),
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_string() {
    let t = Type::ByteString("str".to_string());
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::ByteString(s) => assert_eq!(s, "str"),
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_string_with_whitespaces() {
    let t = Type::ByteString("str   str   str".to_string());
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::ByteString(s) => assert_eq!(s, "str   str   str"),
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_string_with_numbers() {
    let t = Type::ByteString("1222 str 11112".to_string());
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::ByteString(s) => assert_eq!(s, "1222 str 11112"),
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_string_with_newlines() {
    let t = Type::ByteString("1222 str 11112 \r\n 123".to_string());
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::ByteString(s) => assert_eq!(s, "1222 str 11112 \r\n 123"),
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_string_with_delimiter() {
    let t = Type::ByteString(":1".to_string());
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::ByteString(s) => assert_eq!(s, ":1"),
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_string_binary_safe() {
    let t = Type::ByteString("1222 str 11112 \r\n 123 ::: :::".to_string());
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::ByteString(s) => assert_eq!(s, "1222 str 11112 \r\n 123 ::: :::"),
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_empty_sequence() {
    let bytes = "".bytes().collect::<Vec<u8>>();

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "empty byte sequence");
}

#[test]
fn test_decode_start_with_non_digit_bytes() {
    let bytes = "2str:str".bytes().collect::<Vec<u8>>();

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "invalid end byte for type: byte string");
}

#[test]
fn test_decode_missing_length() {
    let bytes = ":24".bytes().collect::<Vec<u8>>();

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "invalid start byte");
}

#[test]
fn test_decode_invalid_length_less() {
    let bytes = "2:1".bytes().collect::<Vec<u8>>();

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "invalid byte string length");
}

#[test]
fn test_decode_invalid_length_more() {
    let bytes = "2:strstr".bytes().collect::<Vec<u8>>();

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "invalid byte sequence");
}

#[test]
fn test_decode_missing_delimiter() {
    let bytes = "2ok".bytes().collect::<Vec<u8>>();

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "invalid end byte for type: byte string");
}

#[test]
fn test_decode_invalid_length_spec() {
    let bytes = "2tt4:121212".bytes().collect::<Vec<u8>>();

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "invalid end byte for type: byte string");
}

#[test]
fn test_decode_invalid_length_spec_missing_digits() {
    let bytes = "str:str".bytes().collect::<Vec<u8>>();

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "invalid start byte");
}
