use std::collections::BTreeMap;

use crate::types::Type;


pub fn encode(t: &Type) -> Vec<u8> {
    match t {
        Type::Integer(i) => encode_integer(i),
        Type::ByteString(s) => encode_bytestring(s),
        Type::List(l) => encode_list(l),
        Type::Dictionary(d) => encode_dictionary(d),
    }
}

fn encode_integer(i: &i64) -> Vec<u8> {
    let mut bytes = vec![b'i'];

    for byte in i.to_string().bytes() {
        bytes.push(byte)
    }
    bytes.push(b'e');

    bytes
}

fn encode_bytestring(s: &String) -> Vec<u8> {
    let mut bytes = vec![];

    for byte in s.len().to_string().bytes() {
        bytes.push(byte)
    }
    bytes.push(b':');

    for byte in s.to_string().bytes() {
        bytes.push(byte)
    }

    bytes
}

fn encode_list(l: &Vec<Type>) -> Vec<u8> {
    let mut bytes = vec![b'l'];

    for t in l {
        bytes.extend(encode(t));
    }
    bytes.push(b'e');

    bytes
}

fn encode_dictionary(d: &BTreeMap<String, Type>) -> Vec<u8> {
    let mut bytes = vec![b'd'];

    for (k, v) in d {
        bytes.extend(encode_bytestring(k));
        bytes.extend(encode(v));
    }
    bytes.push(b'e');

    bytes
}
