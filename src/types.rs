use std::collections::BTreeMap;


pub enum Type {
    Integer(i64),
    ByteString(String),
    List(Vec<Type>),
    Dictionary(BTreeMap<String, Type>),
}
