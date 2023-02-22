use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use crate::BencodeResult;
use crate::protocol::encode;
use crate::types::Type;

pub struct Encoder;

impl Encoder {
    pub fn encode(t: &Type) -> BencodeResult<Vec<u8>> {
        Ok(encode(t))
    }

    pub fn encode_to<P>(t: &Type, path: P) -> BencodeResult<()>
    where
        P: AsRef<Path>
    {
        let mut bytes = encode(t);
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?;
        
        file.write_all(&mut bytes)?;

        Ok(())
    }
}
