mod converter;
mod decode;
mod deserialize;
mod serialize;

pub use converter::ConverterError;
pub use decode::DecodeError;
pub use deserialize::DeserializationError;
pub use serialize::SerializationError;

pub type BencodeError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type BencodeResult<T> = Result<T, BencodeError>;
