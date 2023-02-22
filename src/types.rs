use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use serde::{Deserialize, Serialize};

use crate::errors::serialize::SerializationError;


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Type {
    Integer(i64),
    ByteString(String),
    List(Vec<Type>),
    Dictionary(BTreeMap<String, Type>),
}

impl Type {
    pub fn to_json(&self) -> Result<String, SerializationError>  {
        match serde_json::to_string(self) {
            Err(_) => Err(SerializationError::JsonSerializationError),
            Ok(str) => Ok(str),
        }
    }

    pub fn save_to_json<P>(&self, path: P) -> Result<(), SerializationError>
    where
        P: AsRef<Path>
    {
        let json = self.to_json()?;
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path);

        match file {
            Err(_) => return Err(SerializationError::FileError),
            Ok(mut file) => {
                match writeln!(file, "{}", json) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(SerializationError::FileSerializationError),
                }
            },
        }
    }
}
