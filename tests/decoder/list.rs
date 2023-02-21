use std::collections::BTreeMap;

use bencode::protocol::{decode, encode};
use bencode::types::Type;


#[test]
fn test_decode_empty_list() {
    let t = Type::List(vec![]);
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::List(v) => assert_eq!(v.len(), 0),
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_list_of_integers() {
    let t = Type::List(vec![
        Type::Integer(0),
        Type::Integer(2023),
        Type::Integer(-2023),
    ]);
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::List(v) => {
            assert_eq!(v.len(), 3);

            match &v[0] {
                Type::Integer(i) => assert_eq!(*i, 0),
                _ => panic!("error"),
            }
            match &v[1] {
                Type::Integer(i) => assert_eq!(*i, 2023),
                _ => panic!("error"),
            }
            match &v[2] {
                Type::Integer(i) => assert_eq!(*i, -2023),
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_list_of_binary_strings() {
    let t = Type::List(vec![
        Type::ByteString("123 abc\n".to_string()),
        Type::ByteString("ok".to_string()),
        Type::ByteString("rust".to_string()),
        Type::ByteString("bencode 2023 bencode".to_string()),
    ]);
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::List(v) => {
            assert_eq!(v.len(), 4);

            match &v[0] {
                Type::ByteString(s)=> assert_eq!(*s, "123 abc\n"),
                _ => panic!("error"),
            }
            match &v[1] {
                Type::ByteString(s)=> assert_eq!(*s, "ok"),
                _ => panic!("error"),
            }
            match &v[2] {
                Type::ByteString(s)=> assert_eq!(*s, "rust"),
                _ => panic!("error"),
            }
            match &v[3] {
                Type::ByteString(s)=> assert_eq!(*s, "bencode 2023 bencode"),
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_list_of_integers_and_strings() {
    let t = Type::List(vec![
        Type::ByteString("123 abc\n".to_string()),
        Type::Integer(1),
        Type::ByteString("ok".to_string()),
        Type::ByteString("rust".to_string()),
        Type::ByteString("bencode 2023 bencode".to_string()),
        Type::Integer(-54),
    ]);
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::List(v) => {
            assert_eq!(v.len(), 6);

            match &v[0] {
                Type::ByteString(s)=> assert_eq!(*s, "123 abc\n"),
                _ => panic!("error"),
            }
            match &v[1] {
                Type::Integer(i) => assert_eq!(*i, 1),
                _ => panic!("error"),
            }
            match &v[2] {
                Type::ByteString(s)=> assert_eq!(*s, "ok"),
                _ => panic!("error"),
            }
            match &v[3] {
                Type::ByteString(s)=> assert_eq!(*s, "rust"),
                _ => panic!("error"),
            }
            match &v[4] {
                Type::ByteString(s)=> assert_eq!(*s, "bencode 2023 bencode"),
                _ => panic!("error"),
            }
            match &v[5] {
                Type::Integer(i) => assert_eq!(*i, -54),
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_list_of_integers_and_strings_and_lists() {
    let t = Type::List(vec![
        Type::ByteString("123 abc\n".to_string()),
        Type::Integer(1),
        Type::List(vec![]),
        Type::ByteString("ok".to_string()),
        Type::ByteString("rust".to_string()),
        Type::List(vec![
            Type::ByteString("bytestr".to_string()),
            Type::Integer(202),
        ]),
        Type::ByteString("bencode 2023 bencode".to_string()),
        Type::Integer(-54),
    ]);
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::List(v) => {
            assert_eq!(v.len(), 8);

            match &v[0] {
                Type::ByteString(s)=> assert_eq!(*s, "123 abc\n"),
                _ => panic!("error"),
            }
            match &v[1] {
                Type::Integer(i) => assert_eq!(*i, 1),
                _ => panic!("error"),
            }
            match &v[2] {
                Type::List(l) => assert_eq!(l.len(), 0),
                _ => panic!("error"),
            }
            match &v[3] {
                Type::ByteString(s)=> assert_eq!(*s, "ok"),
                _ => panic!("error"),
            }
            match &v[4] {
                Type::ByteString(s)=> assert_eq!(*s, "rust"),
                _ => panic!("error"),
            }
            match &v[5] {
                Type::List(l) => {
                    assert_eq!(l.len(), 2);

                    match &l[0] {
                        Type::ByteString(s)=> assert_eq!(*s, "bytestr"),
                        _ => panic!("error"),
                    }
                    match &l[1] {
                        Type::Integer(i) => assert_eq!(*i, 202),
                        _ => panic!("error"),
                    }
                },
                _ => panic!("error"),
            }
            match &v[6] {
                Type::ByteString(s)=> assert_eq!(*s, "bencode 2023 bencode"),
                _ => panic!("error"),
            }
            match &v[7] {
                Type::Integer(i) => assert_eq!(*i, -54),
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_list_of_lists() {
    let t = Type::List(vec![
        Type::List(vec![
            Type::ByteString("123 abc\n".to_string()),
            Type::Integer(1),
            Type::ByteString("bencode 2023 bencode".to_string()),
            Type::Integer(-54),
        ]),
        Type::List(vec![
            Type::ByteString("ok".to_string()),
            Type::ByteString("rust".to_string()),
        ]),
        Type::List(vec![
            Type::ByteString("bytestr".to_string()),
            Type::Integer(202),
        ]),
    ]);
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::List(v) => {
            assert_eq!(v.len(), 3);

            match &v[0] {
                Type::List(l) => {
                    assert_eq!(l.len(), 4);
                    match &l[0] {
                        Type::ByteString(s)=> assert_eq!(*s, "123 abc\n"),
                        _ => panic!("error"),
                    }
                    match &l[1] {
                        Type::Integer(i) => assert_eq!(*i, 1),
                        _ => panic!("error"),
                    }
                    match &l[2] {
                        Type::ByteString(s) => assert_eq!(*s, "bencode 2023 bencode"),
                        _ => panic!("error"),
                    }
                    match &l[3] {
                        Type::Integer(i) => assert_eq!(*i, -54),
                        _ => panic!("error"),
                    }
                },
                _ => panic!("error"),
            }
            match &v[1] {
                Type::List(l) => {
                    assert_eq!(l.len(), 2);
                    match &l[0] {
                        Type::ByteString(s)=> assert_eq!(*s, "ok"),
                        _ => panic!("error"),
                    }
                    match &l[1] {
                        Type::ByteString(s) => assert_eq!(*s, "rust"),
                        _ => panic!("error"),
                    }
                },
                _ => panic!("error"),
            }
            match &v[2] {
                Type::List(l) => {
                    assert_eq!(l.len(), 2);

                    match &l[0] {
                        Type::ByteString(s)=> assert_eq!(*s, "bytestr"),
                        _ => panic!("error"),
                    }
                    match &l[1] {
                        Type::Integer(i) => assert_eq!(*i, 202),
                        _ => panic!("error"),
                    }
                },
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_list_of_dictionaries() {
    let t = Type::List(vec![
        Type::Dictionary(BTreeMap::from([
            ("k".to_string(), Type::Integer(1)),
            ("c".to_string(), Type::ByteString("abc".to_string())),
        ])),
        Type::Dictionary(BTreeMap::from([
            ("a".to_string(), Type::Integer(101)),
            ("b".to_string(), Type::Integer(539)),
        ])),
    ]);
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::List(v) => {
            assert_eq!(v.len(), 2);

            match &v[0] {
                Type::Dictionary(d) => {
                    assert_eq!(d.keys().clone().map(|c| c.to_string()).collect::<Vec<String>>(), ["c", "k"]);
                },
                _ => panic!("error"),
            }
            match &v[1] {
                Type::Dictionary(d) => {
                    assert_eq!(d.keys().clone().map(|c| c.to_string()).collect::<Vec<String>>(), ["a", "b"]);
                },
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_list_of_dictionaries_check_values() {
    let t = Type::List(vec![
        Type::Dictionary(BTreeMap::from([
            ("k".to_string(), Type::Integer(1)),
            ("c".to_string(), Type::ByteString("abc".to_string())),
        ])),
        Type::Dictionary(BTreeMap::from([
            ("a".to_string(), Type::Integer(101)),
            ("b".to_string(), Type::Integer(-539)),
        ])),
    ]);
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::List(v) => {
            assert_eq!(v.len(), 2);

            match &v[0] {
                Type::Dictionary(d) => {
                    assert_eq!(d.keys().clone().map(|c| c.to_string()).collect::<Vec<String>>(), ["c", "k"]);
                    let values: Vec<&Type> = d.values().clone().collect();
                    assert_eq!(values.len(), 2);

                    match &values[0] {
                        Type::ByteString(s) => assert_eq!(*s, "abc"),
                        _ => panic!("error"),
                    }
                    match &values[1] {
                        Type::Integer(i) => assert_eq!(*i, 1),
                        _ => panic!("error"),
                    }
                },
                _ => panic!("error"),
            }
            match &v[1] {
                Type::Dictionary(d) => {
                    assert_eq!(d.keys().clone().map(|c| c.to_string()).collect::<Vec<String>>(), ["a", "b"]);
                    let values: Vec<&Type> = d.values().clone().collect();
                    assert_eq!(values.len(), 2);

                    match &values[0] {
                        Type::Integer(i) => assert_eq!(*i, 101),
                        _ => panic!("error"),
                    }
                    match &values[1] {
                        Type::Integer(i) => assert_eq!(*i, -539),
                        _ => panic!("error"),
                    }
                },
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_list_of_dictionaries_check_values_empty_dict() {
    let t = Type::List(vec![
        Type::Dictionary(BTreeMap::from([
            ("k".to_string(), Type::Integer(1)),
            ("c".to_string(), Type::ByteString("abc".to_string())),
        ])),
        Type::Dictionary(BTreeMap::new()),
    ]);
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::List(v) => {
            assert_eq!(v.len(), 2);

            match &v[0] {
                Type::Dictionary(d) => {
                    assert_eq!(d.keys().clone().map(|c| c.to_string()).collect::<Vec<String>>(), ["c", "k"]);
                    let values: Vec<&Type> = d.values().clone().collect();
                    assert_eq!(values.len(), 2);

                    match &values[0] {
                        Type::ByteString(s) => assert_eq!(*s, "abc"),
                        _ => panic!("error"),
                    }
                    match &values[1] {
                        Type::Integer(i) => assert_eq!(*i, 1),
                        _ => panic!("error"),
                    }
                },
                _ => panic!("error"),
            }
            match &v[1] {
                Type::Dictionary(d) => {
                    let keys = d.keys().clone().map(|c| c.to_string()).collect::<Vec<String>>();
                    assert_eq!(keys.len(), 0);
                    let values: Vec<&Type> = d.values().clone().collect();
                    assert_eq!(values.len(), 0);
                },
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_list_all() {
    let t = Type::List(vec![
        Type::Dictionary(BTreeMap::from([
            ("k".to_string(), Type::Integer(1)),
            ("c".to_string(), Type::ByteString("abc".to_string())),
        ])),
        Type::Dictionary(BTreeMap::from([
            ("a".to_string(), Type::Integer(101)),
            ("b".to_string(), Type::Integer(-539)),
        ])),
        Type::List(vec![
            Type::ByteString("123 abc\n".to_string()),
            Type::Integer(1),
            Type::ByteString("bencode 2023 bencode".to_string()),
            Type::Integer(-54),
        ]),
    ]);
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::List(v) => {
            assert_eq!(v.len(), 3);

            match &v[0] {
                Type::Dictionary(d) => {
                    assert_eq!(d.keys().clone().map(|c| c.to_string()).collect::<Vec<String>>(), ["c", "k"]);
                    let values: Vec<&Type> = d.values().clone().collect();
                    assert_eq!(values.len(), 2);

                    match &values[0] {
                        Type::ByteString(s) => assert_eq!(*s, "abc"),
                        _ => panic!("error"),
                    }
                    match &values[1] {
                        Type::Integer(i) => assert_eq!(*i, 1),
                        _ => panic!("error"),
                    }
                },
                _ => panic!("error"),
            }
            match &v[1] {
                Type::Dictionary(d) => {
                    assert_eq!(d.keys().clone().map(|c| c.to_string()).collect::<Vec<String>>(), ["a", "b"]);
                    let values: Vec<&Type> = d.values().clone().collect();
                    assert_eq!(values.len(), 2);

                    match &values[0] {
                        Type::Integer(i) => assert_eq!(*i, 101),
                        _ => panic!("error"),
                    }
                    match &values[1] {
                        Type::Integer(i) => assert_eq!(*i, -539),
                        _ => panic!("error"),
                    }
                },
                _ => panic!("error"),
            }
            match &v[2] {
                Type::List(l) => {
                    assert_eq!(l.len(), 4);
                    match &l[0] {
                        Type::ByteString(s)=> assert_eq!(*s, "123 abc\n"),
                        _ => panic!("error"),
                    }
                    match &l[1] {
                        Type::Integer(i) => assert_eq!(*i, 1),
                        _ => panic!("error"),
                    }
                    match &l[2] {
                        Type::ByteString(s) => assert_eq!(*s, "bencode 2023 bencode"),
                        _ => panic!("error"),
                    }
                    match &l[3] {
                        Type::Integer(i) => assert_eq!(*i, -54),
                        _ => panic!("error"),
                    }
                },
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}
