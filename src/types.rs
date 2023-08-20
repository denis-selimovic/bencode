use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::errors::{ConverterError, DeserializationError, SerializationError};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Type {
    Integer(i64),
    ByteString(String),
    List(Vec<Type>),
    Dictionary(BTreeMap<String, Type>),
}

impl TryFrom<Type> for String {
    type Error = ConverterError;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        match value {
            Type::ByteString(s) => Ok(s),
            _ => Err(ConverterError::InvalidString),
        }
    }
}

impl TryFrom<&Type> for String {
    type Error = ConverterError;

    fn try_from(value: &Type) -> Result<Self, Self::Error> {
        match value {
            Type::ByteString(s) => Ok(s.to_string()),
            _ => Err(ConverterError::InvalidString),
        }
    }
}

impl TryFrom<Type> for i64 {
    type Error = ConverterError;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        match value {
            Type::Integer(i) => Ok(i),
            _ => Err(ConverterError::InvalidInteger),
        }
    }
}

impl Type {
    pub fn from_json(s: &String) -> Result<Type, DeserializationError> {
        match serde_json::from_str::<Type>(s) {
            Err(_) => Err(DeserializationError::JsonDeserializationError),
            Ok(t) => Ok(t),
        }
    }

    pub fn to_json(&self) -> Result<String, SerializationError> {
        match serde_json::to_string(self) {
            Err(_) => Err(SerializationError::JsonSerializationError),
            Ok(str) => Ok(str),
        }
    }

    pub fn load_from_json<P>(path: P) -> Result<Type, DeserializationError>
    where
        P: AsRef<Path>,
    {
        match read_to_string(path) {
            Err(_) => return Err(DeserializationError::FileError),
            Ok(json_str) => Type::from_json(&json_str),
        }
    }

    pub fn save_to_json<P>(&self, path: P) -> Result<(), SerializationError>
    where
        P: AsRef<Path>,
    {
        let json = self.to_json()?;
        let file = OpenOptions::new().write(true).create_new(true).open(path);

        match file {
            Err(_) => return Err(SerializationError::FileError),
            Ok(mut file) => match writeln!(file, "{}", json) {
                Ok(_) => Ok(()),
                Err(_) => Err(SerializationError::FileSerializationError),
            },
        }
    }

    pub fn get(&self, key: String) -> Result<&Type, ConverterError> {
        match self {
            Type::Dictionary(d) => match d.get(&key) {
                Some(t) => Ok(t),
                None => Err(ConverterError::InvalidDictionary),
            },
            _ => Err(ConverterError::InvalidDictionary),
        }
    }
}
