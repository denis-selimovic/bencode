use std::collections::BTreeMap;

use bencode::protocol::{decode, encode};
use bencode::types::Type;


#[test]
fn test_decode_dictionary_list() {
    let t = Type::Dictionary(BTreeMap::new());
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Dictionary(d) => assert_eq!(d.len(), 0),
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_dictionary_integers() {
    let t = Type::Dictionary(BTreeMap::from([
        ("c".to_string(), Type::Integer(1)),
        ("a".to_string(), Type::Integer(2)),
        ("aaa".to_string(), Type::Integer(4)),
    ]));
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Dictionary(d) => {
            assert_eq!(d.len(), 3);
            assert_eq!(d.keys().clone().map(|c| c.to_string()).collect::<Vec<String>>(), ["a", "aaa", "c"]);
            let values: Vec<&Type> = d.values().clone().collect();

            match &values[0] {
                Type::Integer(i) => assert_eq!(*i, 2),
                _ => panic!("error"),
            }
            match &values[1] {
                Type::Integer(i) => assert_eq!(*i, 4),
                _ => panic!("error"),
            }
            match &values[2] {
                Type::Integer(i) => assert_eq!(*i, 1),
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_dictionary_strings() {
    let t = Type::Dictionary(BTreeMap::from([
        ("bbb".to_string(), Type::ByteString("val".to_string())),
        ("b".to_string(), Type::ByteString("val4".to_string())),
        ("bb".to_string(), Type::ByteString("val2".to_string())),
    ]));
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Dictionary(d) => {
            assert_eq!(d.len(), 3);
            assert_eq!(d.keys().clone().map(|c| c.to_string()).collect::<Vec<String>>(), ["b", "bb", "bbb"]);
            let values: Vec<&Type> = d.values().clone().collect();

            match &values[0] {
                Type::ByteString(s) => assert_eq!(*s, "val4"),
                _ => panic!("error"),
            }
            match &values[1] {
                Type::ByteString(s) => assert_eq!(*s, "val2"),
                _ => panic!("error"),
            }
            match &values[2] {
                Type::ByteString(s) => assert_eq!(*s, "val"),
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_dictionary_integers_strings() {
    let t = Type::Dictionary(BTreeMap::from([
        ("bbb".to_string(), Type::ByteString("val".to_string())),
        ("b".to_string(), Type::ByteString("val4".to_string())),
        ("bb".to_string(), Type::ByteString("val2".to_string())),
        ("c".to_string(), Type::Integer(1)),
        ("a".to_string(), Type::Integer(2)),
        ("aaa".to_string(), Type::Integer(4)),
    ]));
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Dictionary(d) => {
            assert_eq!(d.len(), 6);
            assert_eq!(
                d.keys().clone().map(|c| c.to_string()).collect::<Vec<String>>(),
                ["a", "aaa", "b", "bb", "bbb", "c"],
            );

            let values: Vec<&Type> = d.values().clone().collect();

            match &values[0] {
                Type::Integer(i) => assert_eq!(*i, 2),
                _ => panic!("error"),
            }
            match &values[1] {
                Type::Integer(i) => assert_eq!(*i, 4),
                _ => panic!("error"),
            }
            match &values[2] {
                Type::ByteString(s) => assert_eq!(*s, "val4"),
                _ => panic!("error"),
            }
            match &values[3] {
                Type::ByteString(s) => assert_eq!(*s, "val2"),
                _ => panic!("error"),
            }
            match &values[4] {
                Type::ByteString(s) => assert_eq!(*s, "val"),
                _ => panic!("error"),
            }
            match &values[5] {
                Type::Integer(i) => assert_eq!(*i, 1),
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}

#[test]
fn test_decode_dictionary_integers_strings_lists() {
    let t = Type::Dictionary(BTreeMap::from([
        ("bbb".to_string(), Type::ByteString("val".to_string())),
        ("b".to_string(), Type::ByteString("val4".to_string())),
        ("bb".to_string(), Type::ByteString("val2".to_string())),
        ("c".to_string(), Type::Integer(1)),
        ("a".to_string(), Type::Integer(2)),
        ("aaa".to_string(), Type::Integer(4)),
        ("l1".to_string(), Type::List(vec![])),
        ("l2".to_string(), Type::List(vec![Type::Integer(-1), Type::Dictionary(BTreeMap::new())])),
    ]));
    let bytes = encode(&t);

    let res = decode(&mut bytes.into_iter());
    assert_eq!(res.is_ok(), true);

    match res.ok().unwrap() {
        Type::Dictionary(d) => {
            assert_eq!(d.len(), 8);
            assert_eq!(
                d.keys().clone().map(|c| c.to_string()).collect::<Vec<String>>(),
                ["a", "aaa", "b", "bb", "bbb", "c", "l1", "l2"],
            );

            let values: Vec<&Type> = d.values().clone().collect();

            match &values[0] {
                Type::Integer(i) => assert_eq!(*i, 2),
                _ => panic!("error"),
            }
            match &values[1] {
                Type::Integer(i) => assert_eq!(*i, 4),
                _ => panic!("error"),
            }
            match &values[2] {
                Type::ByteString(s) => assert_eq!(*s, "val4"),
                _ => panic!("error"),
            }
            match &values[3] {
                Type::ByteString(s) => assert_eq!(*s, "val2"),
                _ => panic!("error"),
            }
            match &values[4] {
                Type::ByteString(s) => assert_eq!(*s, "val"),
                _ => panic!("error"),
            }
            match &values[5] {
                Type::Integer(i) => assert_eq!(*i, 1),
                _ => panic!("error"),
            }
            match &values[6] {
                Type::List(l) => assert_eq!(l.len(), 0),
                _ => panic!("error"),
            }
            match &values[7] {
                Type::List(l) => {
                    assert_eq!(l.len(), 2);

                    match &l[0] {
                        Type::Integer(i) => assert_eq!(*i, -1),
                        _ => panic!("error"),
                    }
                    match &l[1] {
                        Type::Dictionary(d) => assert_eq!(d.len(), 0),
                        _ => panic!("error"),
                    }
                },
                _ => panic!("error"),
            }
        },
        _ => panic!("error"),
    }
}
