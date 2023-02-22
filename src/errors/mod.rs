mod decode;
mod serialize;

pub use decode::DecodeError;
pub use serialize::SerializationError;

pub type BencodeError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type BencodeResult<T> = Result<T, BencodeError>;
