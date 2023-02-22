use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::errors::BencodeResult;
use crate::protocol::decode;
use crate::types::Type;

pub struct Decoder;

impl Decoder {
    pub fn decode<I>(it: &mut I) -> BencodeResult<Type>
    where
        I: Iterator<Item = u8>
    {
        match decode(it) {
            Err(err) => Err(err.into()),
            Ok(t) => Ok(t),
        }
    }

    pub fn decode_from<P>(path: P) -> BencodeResult<Type>
    where
        P: AsRef<Path>
    {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file).bytes().map(|c| c.unwrap());
        let decoded = decode(&mut reader)?;

        Ok(decoded)
    }
}
