use std::string::String;
use std::vec::Vec;

// use anymap::AnyMap;
// use std::any::Any;

use types::BertTag;


pub struct Serialiazer {
//    data: Any
}


pub struct Deserializer {
//    data: Vec<u8>
}


pub trait Serialize<T> {
    fn to_bert(&self, data: T) -> Vec<u8>;
}


pub trait Deserialize<T> {
    fn to_bert(&self, data: T) -> Vec<u8>;
}


impl Serialize<String> for Serialiazer {
    fn to_bert(&self, data: String) -> Vec<u8> {
        let tag = BertTag::String as u8;
        let binary_string = data.into_bytes();
        let binary_length = binary_string.len() as u8;

        let mut binary: Vec<u8> = vec![tag, 0, binary_length];
        binary.extend(binary_string.iter().clone());

        binary
    }
}


impl Deserializer {

}
