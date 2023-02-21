use std::collections::BTreeMap;

use bencode::protocol::encode;
use bencode::types::Type;


#[test]
fn test_encode_positive_integer() {
    let correct = "i42e".bytes().collect::<Vec<u8>>();
    let to_encode = Type::Integer(42);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_zer0() {
    let correct = "i0e".bytes().collect::<Vec<u8>>();
    let to_encode = Type::Integer(0);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_negative_integer() {
    let correct = "i-204e".bytes().collect::<Vec<u8>>();
    let to_encode = Type::Integer(-204);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_positive_upper_bound() {
    let correct = format!("i{}e", i64::MAX).bytes().collect::<Vec<u8>>();
    let to_encode = Type::Integer(i64::MAX);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_negative_lower_bound() {
    let correct = format!("i{}e", i64::MIN).bytes().collect::<Vec<u8>>();
    let to_encode = Type::Integer(i64::MIN);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_empty_string() {
    let correct = "0:".bytes().collect::<Vec<u8>>();
    let to_encode = Type::ByteString("".to_string());

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_string() {
    let correct = "10:spamsspams".bytes().collect::<Vec<u8>>();
    let to_encode = Type::ByteString("spamsspams".to_string());

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_string_one_byte() {
    let correct = "1:c".bytes().collect::<Vec<u8>>();
    let to_encode = Type::ByteString("c".to_string());

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_empty_multiple_bytes() {
    let correct = "26:abcdefghijklmnopqrstuvwxyz".bytes().collect::<Vec<u8>>();
    let to_encode = Type::ByteString("abcdefghijklmnopqrstuvwxyz".to_string());

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_string_very_login() {
    let correct = "78:abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz"
                            .bytes().collect::<Vec<u8>>();
    let to_encode = Type::ByteString(
        "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_string()
);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_list_empty() {
    let correct = "le".bytes().collect::<Vec<u8>>();
    let l = vec![];
    let to_encode = Type::List(l);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_list_of_integers() {
    let correct = "li42ei0ei-204ee".bytes().collect::<Vec<u8>>();
    let l = vec![Type::Integer(42), Type::Integer(0), Type::Integer(-204)];
    let to_encode = Type::List(l);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_list_of_strings() {
    let correct = "l3:str1:x3:abce".bytes().collect::<Vec<u8>>();
    let l = vec![
        Type::ByteString("str".to_string()),
        Type::ByteString("x".to_string()),
        Type::ByteString("abc".to_string()),
    ];
    let to_encode = Type::List(l);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_list_of_integers_and_strings() {
    let correct = "l3:stri42ei0e1:x3:abci-55ee".bytes().collect::<Vec<u8>>();
    let l = vec![
        Type::ByteString("str".to_string()),
        Type::Integer(42),
        Type::Integer(0),
        Type::ByteString("x".to_string()),
        Type::ByteString("abc".to_string()),
        Type::Integer(-55),
    ];
    let to_encode = Type::List(l);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_list_of_integers_and_strings_large() {
    let correct = "l3:stri33ei0e1:x3:abci-1111e2:ok3:okee".bytes().collect::<Vec<u8>>();
    let l = vec![
        Type::ByteString("str".to_string()),
        Type::Integer(33),
        Type::Integer(0),
        Type::ByteString("x".to_string()),
        Type::ByteString("abc".to_string()),
        Type::Integer(-1111),
        Type::ByteString("ok".to_string()),
        Type::ByteString("oke".to_string()),
    ];
    let to_encode = Type::List(l);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_list_of_integers_and_lists() {
    let correct = "li51eli2ei10eei111eli-5ei-11ei0eelee".bytes().collect::<Vec<u8>>();
    let l = vec![
        Type::Integer(51),
        Type::List(vec![Type::Integer(2), Type::Integer(10)]),
        Type::Integer(111),
        Type::List(vec![Type::Integer(-5), Type::Integer(-11), Type::Integer(0)]),
        Type::List(vec![]),
    ];
    let to_encode = Type::List(l);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_list_of_strings_and_lists() {
    let correct = "l3:abcl3:def1:3e6:ghimnol0:elee".bytes().collect::<Vec<u8>>();
    let l = vec![
        Type::ByteString("abc".to_string()),
        Type::List(vec![Type::ByteString("def".to_string()), Type::ByteString("3".to_string())]),
        Type::ByteString("ghimno".to_string()),
        Type::List(vec![Type::ByteString("".to_string())]),
        Type::List(vec![]),
    ];
    let to_encode = Type::List(l);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_list_of_integers_strings_and_lists() {
    let correct = "l3:abcl3:def1:3ei2023el0:eli1e4:rustee".bytes().collect::<Vec<u8>>();
    let l = vec![
        Type::ByteString("abc".to_string()),
        Type::List(vec![Type::ByteString("def".to_string()), Type::ByteString("3".to_string())]),
        Type::Integer(2023),
        Type::List(vec![Type::ByteString("".to_string())]),
        Type::List(vec![Type::Integer(1), Type::ByteString("rust".to_string())]),
    ];
    let to_encode = Type::List(l);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_list_of_lists() {
    let correct = "ll3:def1:3el0:eli1e4:rustee".bytes().collect::<Vec<u8>>();
    let l = vec![
        Type::List(vec![Type::ByteString("def".to_string()), Type::ByteString("3".to_string())]),
        Type::List(vec![Type::ByteString("".to_string())]),
        Type::List(vec![Type::Integer(1), Type::ByteString("rust".to_string())]),
    ];
    let to_encode = Type::List(l);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_list_of_mixed_types() {
    let correct = "l3:abcl3:def1:3ei2023el0:eli1e4:rusted1:3i1e3:key3:valee".bytes().collect::<Vec<u8>>();
    let l = vec![
        Type::ByteString("abc".to_string()),
        Type::List(vec![Type::ByteString("def".to_string()), Type::ByteString("3".to_string())]),
        Type::Integer(2023),
        Type::List(vec![Type::ByteString("".to_string())]),
        Type::List(vec![Type::Integer(1), Type::ByteString("rust".to_string())]),
        Type::Dictionary(BTreeMap::from(
            [
                ("3".to_string(), Type::Integer(1)),
                ("key".to_string(), Type::ByteString("val".to_string()))
            ]
    ))
    ];
    let to_encode = Type::List(l);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_dictionary_empty() {
    let correct = "de".bytes().collect::<Vec<u8>>();
    let d = BTreeMap::new();
    let to_encode = Type::Dictionary(d);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_dictionary_of_integers() {
    let correct = "d2:k1i5e2:k2i10e2:k3i1ee".bytes().collect::<Vec<u8>>();
    let d = BTreeMap::from([
        ("k3".to_string(), Type::Integer(1)),
        ("k1".to_string(), Type::Integer(5)),
        ("k2".to_string(), Type::Integer(10)),
    ]);
    let to_encode = Type::Dictionary(d);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_dictionary_of_strings() {
    let correct = "d2:k12:v32:k22:v22:k32:v42:k42:v1e".bytes().collect::<Vec<u8>>();
    let d = BTreeMap::from([
        ("k4".to_string(), Type::ByteString("v1".to_string())),
        ("k2".to_string(), Type::ByteString("v2".to_string())),
        ("k1".to_string(), Type::ByteString("v3".to_string())),
        ("k3".to_string(), Type::ByteString("v4".to_string())),
    ]);
    let to_encode = Type::Dictionary(d);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_dictionary_of_integers_and_strings() {
    let correct = "d2:k1i5e2:k22:v22:k32:v42:k4i-10ee".bytes().collect::<Vec<u8>>();
    let d = BTreeMap::from([
        ("k4".to_string(), Type::Integer(-10)),
        ("k2".to_string(), Type::ByteString("v2".to_string())),
        ("k1".to_string(), Type::Integer(5)),
        ("k3".to_string(), Type::ByteString("v4".to_string())),
    ]);
    let to_encode = Type::Dictionary(d);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_dictionary_of_multiple_integers_and_strings() {
    let correct = "d1:a4:aval1:b4:bval1:ci100e2:k1i5e2:k22:v22:k32:v42:k4i-10ee".bytes().collect::<Vec<u8>>();
    let d = BTreeMap::from([
        ("k4".to_string(), Type::Integer(-10)),
        ("k2".to_string(), Type::ByteString("v2".to_string())),
        ("k1".to_string(), Type::Integer(5)),
        ("k3".to_string(), Type::ByteString("v4".to_string())),
        ("a".to_string(), Type::ByteString("aval".to_string())),
        ("b".to_string(), Type::ByteString("bval".to_string())),
        ("c".to_string(), Type::Integer(100)),
    ]);
    let to_encode = Type::Dictionary(d);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_dictionary_of_lists_of_integers() {
    let correct = "d2:k2li2ei10ei-1ee2:k4li1ei2eee".bytes().collect::<Vec<u8>>();
    let d = BTreeMap::from([
        ("k4".to_string(), Type::List(vec![Type::Integer(1), Type::Integer(2)])),
        ("k2".to_string(), Type::List(vec![Type::Integer(2), Type::Integer(10), Type::Integer(-1)])),
    ]);
    let to_encode = Type::Dictionary(d);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_dictionary_of_lists_of_strings() {
    let correct = "d2:k1le2:k3l3:abce2:k5l3:def3:ghiee".bytes().collect::<Vec<u8>>();
    let d = BTreeMap::from([
        ("k3".to_string(), Type::List(vec![Type::ByteString("abc".to_string())])),
        ("k1".to_string(), Type::List(vec![])),
        ("k5".to_string(), Type::List(vec![Type::ByteString("def".to_string()), Type::ByteString("ghi".to_string())])),
    ]);
    let to_encode = Type::Dictionary(d);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_dictionary_of_lists_of_integers_and_strings() {
    let correct = "d2:k1li-11e2:oke2:k3l3:abce2:k5l3:def3:ghiee".bytes().collect::<Vec<u8>>();
    let d = BTreeMap::from([
        ("k3".to_string(), Type::List(vec![Type::ByteString("abc".to_string())])),
        ("k1".to_string(), Type::List(vec![Type::Integer(-11), Type::ByteString("ok".to_string())])),
        ("k5".to_string(), Type::List(vec![Type::ByteString("def".to_string()), Type::ByteString("ghi".to_string())])),
    ]);
    let to_encode = Type::Dictionary(d);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_dictionary_of_lists_of_integers_and_strings_and_dictionaries() {
    let correct = "d1:dd1:ki1ee2:k1li-11e2:oke2:k3l3:abce2:k5l3:def3:ghiee".bytes().collect::<Vec<u8>>();
    let d = BTreeMap::from([
        ("k3".to_string(), Type::List(vec![Type::ByteString("abc".to_string())])),
        ("k1".to_string(), Type::List(vec![Type::Integer(-11), Type::ByteString("ok".to_string())])),
        ("k5".to_string(), Type::List(vec![Type::ByteString("def".to_string()), Type::ByteString("ghi".to_string())])),
        ("d".to_string(), Type::Dictionary(BTreeMap::from([("k".to_string(), Type::Integer(1))]))),
    ]);
    let to_encode = Type::Dictionary(d);

    assert_eq!(encode(&to_encode), correct);
}

#[test]
fn test_encode_dictionary_of_dictionaries() {
    let correct = "d1:dd2:k1i1ee1:fd2:k22:ffee".bytes().collect::<Vec<u8>>();
    let d = BTreeMap::from([
        ("f".to_string(), Type::Dictionary(BTreeMap::from([("k2".to_string(), Type::ByteString("ff".to_string()))]))),
        ("d".to_string(), Type::Dictionary(BTreeMap::from([("k1".to_string(), Type::Integer(1))]))),
    ]);
    let to_encode = Type::Dictionary(d);

    assert_eq!(encode(&to_encode), correct);
}

