use std::collections::BTreeMap;

use crate::errors::decode::DecodeError;
use crate::types::Type;


pub type DecodeResult = Result<Type, DecodeError>;


pub fn decode<T>(bytes: &mut T) -> DecodeResult
where
    T: Iterator<Item = u8>
{
    let result = match bytes.next() {
        None => return Err(DecodeError::Empty),
        Some(start_byte) => handler(bytes, start_byte),
    };

    if result.is_err() {
        return result;
    }

    match bytes.next() {
        None => result,
        Some(_) => return Err(DecodeError::InvalidByteSequence),
    }
}

fn handler<T>(bytes: &mut T, start_byte: u8) -> DecodeResult
where
    T: Iterator<Item = u8>
{
    match start_byte {
        b'i' => decode_integer(bytes, start_byte),
        b'l' => decode_list(bytes, start_byte),
        b'd' => decode_dictionary(bytes, start_byte),
        b'0'..=b'9' => decode_binarystring(bytes, start_byte),
        _ => return Err(DecodeError::InvalidStartByte),
    }
}

fn decode_integer<T>(bytes: &mut T, _start_byte: u8) -> DecodeResult
where
    T: Iterator<Item = u8>
{
    let mut buff = vec![];
    let mut sign = 1;

    let nxt = match bytes.next() {
        None => return Err(DecodeError::InvalidInteger),
        Some(ch) => ch, 
    };

    if nxt == b'-' {
        sign = -1;
    } else if nxt >= b'0' && nxt <= b'9' {
        buff.push(nxt);
    } else {
        return Err(DecodeError::InvalidInteger);
    }

    while let Some(ch) = bytes.next() {
        match ch {
            b'0'..=b'9' => buff.push(ch),
            b'e' => break,
            _ => return Err(DecodeError::InvalidEndByte("integer".to_string())),
        }
    }

    if buff.len() > 1 && buff[0] == b'0' {
        return Err(DecodeError::IntegerWithLeadingZeros);
    }

    let i = bytes_to_int(buff)?;

    if sign == -1 && i == 0 {
        return Err(DecodeError::NegativeZeroInteger);
    }

    Ok(Type::Integer(sign * i))
}

fn decode_binarystring<T>(bytes: &mut T, start_byte: u8) -> DecodeResult
where
    T: Iterator<Item = u8>
{
    let mut len_buff = vec![start_byte];
    let mut str_buff = vec![];

    while let Some(ch) = bytes.next() {
        match ch {
            b'0'..=b'9' => len_buff.push(ch),
            b':' => break,
            _ => return Err(DecodeError::InvalidEndByte("byte string".to_string())),
        }
    }

    let len = bytes_to_int(len_buff)?;
    
    for _ in 0..len {
        match bytes.next() {
            None => return Err(DecodeError::InvalidByteStringLength),
            Some(ch) => str_buff.push(ch),
        }
    }

    Ok(Type::ByteString(bytes_to_str(str_buff)))
}

fn decode_list<T>(bytes: &mut T, _start_byte: u8) -> DecodeResult
where
    T: Iterator<Item = u8>
{
    let mut l = vec![];

    loop {
        match bytes.next() {
            None => return Err(DecodeError::InvalidList),
            Some(ch) => {
                match ch {
                    b'e' => break,
                    _ => {
                        let item = decode(bytes)?;
                        l.push(item);
                    }
                }
            }
        }
    }

    Ok(Type::List(l))
}

fn decode_dictionary<T>(bytes: &mut T, _start_byte: u8) -> DecodeResult
where
    T: Iterator<Item = u8>
{
    let mut d = BTreeMap::new();

    loop {
        match bytes.next() {
            None => return Err(DecodeError::InvalidDictionary),
            Some(ch) => {
                match ch {
                    b'e' => break,
                    _ => {
                        let key = decode_binarystring(bytes, ch)?;
                        let value = decode(bytes)?;
                        
                        match key {
                            Type::ByteString(key) => d.insert(key, value),
                            _ => return Err(DecodeError::InvalidDictionaryKey),
                        };
                    }
                }
            }
        }
    }

    Ok(Type::Dictionary(d))
}

fn bytes_to_str(bytes: Vec<u8>) -> String { 
    bytes.iter().map(|&b| b as char).collect::<String>()
}

fn bytes_to_int(bytes: Vec<u8>) -> Result<i64, DecodeError> {
    let integer_str = bytes_to_str(bytes);

    match integer_str.parse::<i64>() {
        Err(_) => return Err(DecodeError::InvalidInteger),
        Ok(i) => Ok(i),
    }
}
