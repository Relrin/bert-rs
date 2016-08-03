use std::string::String;
use std::vec::Vec;

// use anymap::AnyMap;
use byteorder::{BigEndian, WriteBytesExt};
// use std::any::Any;

use types::{BERT_LABEL, BertTag};


pub struct Serializer {
//    data: Any
}


pub struct Deserializer {
//    data: Vec<u8>
}


pub trait Serialize<T> {
    fn to_bert(&self, data: T) -> Vec<u8>;
}


trait Deserialize<T> {
    fn to_bert(&self, data: T) -> Vec<u8>;
}


impl Serializer {

    pub fn generate_term(&self, tag: BertTag, data: Vec<u8>) -> Vec<u8> {
        let mut binary = vec![tag as u8];
        binary.extend(data.iter().clone());
        binary
    }

    pub fn convert_string_to_binary(&self, data: &str) -> Vec<u8> {
        let binary_string = data.as_bytes();
        let binary_length = binary_string.len() as u8;
        let mut binary = vec![0u8, binary_length];
        binary.extend(binary_string.iter().clone());
        binary
    }

    pub fn merge_atoms(&self, atom_1: Vec<u8>, atom_2: Vec<u8>) -> Vec<u8> {
        let mut binary: Vec<u8> = atom_1.clone();
        binary.extend(atom_2.iter().clone());
        binary
    }

    pub fn get_bert_atom(&self) -> Vec<u8> {
        let binary_string = self.convert_string_to_binary(BERT_LABEL);
        self.generate_term(BertTag::Atom, binary_string)
    }
}


impl Serialize<u8> for Serializer {
    fn to_bert(&self, data: u8) -> Vec<u8> {
        self.generate_term(BertTag::SmallInteger, vec![data])
    }
}


impl Serialize<i32> for Serializer {
    fn to_bert(&self, data: i32) -> Vec<u8> {
        let mut binary = vec![];
        binary.write_i32::<BigEndian>(data).unwrap();
        self.generate_term(BertTag::Integer, binary)
    }
}


impl Serialize<f64> for Serializer {
    fn to_bert(&self, data: f64) -> Vec<u8> {
        let mut binary = vec![];
        binary.write_f64::<BigEndian>(data).unwrap();
        self.generate_term(BertTag::Float, binary)
    }
}


impl Serialize<String> for Serializer {
    fn to_bert(&self, data: String) -> Vec<u8> {
        let binary_string = self.convert_string_to_binary(&data);
        self.generate_term(BertTag::String, binary_string)
    }
}


impl Serialize<bool> for Serializer {
    fn to_bert(&self, data: bool) -> Vec<u8> {
        let boolean_string = data.to_string();
        let binary_boolean = self.convert_string_to_binary(&boolean_string);

        let bert_atom = self.get_bert_atom();
        let boolean_atom = self.generate_term(BertTag::Atom, binary_boolean);

        self.merge_atoms(bert_atom, boolean_atom)
    }
}


#[cfg(test)]
mod test {
    use super::{Serializer};

    #[test]
    fn test_serialize_bool() {
        let serializer = Serializer{};

        println!(serializer.to_bert(true), [100, 0, 4, 116, 114, 117, 101])
    }
}
