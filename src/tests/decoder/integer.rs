use crate::protocol::{decode, encode};
use crate::Type;

#[test]
fn test_decode_zero() {
    let t = Type::Integer(0);
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Integer(i) => assert_eq!(i, 0),
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_positive_number() {
    let t = Type::Integer(2023);
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Integer(i) => assert_eq!(i, 2023),
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_negative_number() {
    let t = Type::Integer(-2023);
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Integer(i) => assert_eq!(i, -2023),
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_max_integer() {
    let t = Type::Integer(i64::MAX);
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Integer(i) => assert_eq!(i, i64::MAX),
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_min_integer() {
    let t = Type::Integer(i64::MIN + 1);
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Integer(i) => assert_eq!(i, i64::MIN + 1),
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_only_minus() {
    let bytes = "i-e".bytes().collect::<Vec<u8>>();

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "invalid integer");
}

#[test]
fn test_decode_invalid_integer_with_letters() {
    let bytes = "i100str200e".bytes().collect::<Vec<u8>>();

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "invalid end byte for type: integer");
}

#[test]
fn test_decode_invalid_start() {
    let bytes = "is100e".bytes().collect::<Vec<u8>>();

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "invalid integer");
}

#[test]
fn test_decode_only_start_byte() {
    let bytes = "i".bytes().collect::<Vec<u8>>();

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "invalid integer");
}

#[test]
fn test_decode_negative_zero() {
    let bytes = "i-0e".bytes().collect::<Vec<u8>>();

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "negative zero is not allowed");
}

#[test]
fn test_decode_leading_zeros() {
    let bytes = "i0100e".bytes().collect::<Vec<u8>>();

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "integer with leading zeros is not allowed");
}

#[test]
fn test_decode_multiple_leading_zeros() {
    let bytes = "i000100e".bytes().collect::<Vec<u8>>();

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "integer with leading zeros is not allowed");
}

#[test]
fn test_decode_wrong_start_byte() {
    let bytes = "+0e".bytes().collect::<Vec<u8>>();

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "invalid start byte");
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
fn test_decode_extra_bytes() {
    let bytes = "i100ei".bytes().collect::<Vec<u8>>();

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_err(), true);

    let err = res.err().unwrap();
    assert_eq!(err.to_string(), "invalid byte sequence");
}
