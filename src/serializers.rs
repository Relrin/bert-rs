use std::string::String;
use std::vec::Vec;

use byteorder::{BigEndian, WriteBytesExt};
use types::{BERT_LABEL, EXT_VERSION, BertTag, BertType, BertTuple};


pub struct Serializer;


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
    pub fn new() -> Serializer {
        Serializer {}
    }

    pub fn term_to_binary<T>(&self, data: T) -> Vec<u8> where Self: Serialize<T> {
        let mut binary = vec![EXT_VERSION];
        let serialized_data = self.to_bert(data);
        binary.extend(serialized_data.iter().clone());
        binary
    }

    fn generate_term(&self, tag: BertTag, data: Vec<u8>) -> Vec<u8> {
        let mut binary = vec![tag as u8];
        binary.extend(data.iter().clone());
        binary
    }

    fn convert_string_to_binary(&self, data: &str) -> Vec<u8> {
        let binary_string = data.as_bytes();
        let binary_length = binary_string.len() as i16;
        let mut binary = vec![];
        binary.write_i16::<BigEndian>(binary_length).unwrap();
        binary.extend(binary_string.iter().clone());
        binary
    }

    fn merge_terms(&self, term_1: Vec<u8>, term_2: Vec<u8>) -> Vec<u8> {
        let mut binary: Vec<u8> = term_1.clone();
        binary.extend(term_2.iter().clone());
        binary
    }

    fn get_atom(&self, name: &str) -> Vec<u8> {
        let name: Vec<u8> = self.convert_string_to_binary(name);
        self.generate_term(BertTag::Atom, name)
    }

    fn get_nil(&self) -> Vec<u8> {
        vec![BertTag::Nil as u8]
    }

    fn get_bert_nil(&self) -> Vec<u8> {
        let bert_atom = self.get_bert_atom();
        let nil_atom = self.get_atom("nil");

        let mut binary: Vec<u8> = vec![];
        binary.extend(bert_atom.iter().clone());
        binary.extend(nil_atom.iter().clone());
        self.get_small_tuple(2, binary)
    }

    fn get_bert_atom(&self) -> Vec<u8> {
        self.get_atom(BERT_LABEL)
    }

    fn get_small_tuple(&self, arity: u8, elements: Vec<u8>) -> Vec<u8> {
        let mut binary = vec![arity];
        binary.extend(elements.iter().clone());
        self.generate_term(BertTag::SmallTuple, binary)
    }

    fn get_large_tuple(&self, arity: i32, elements: Vec<u8>) -> Vec<u8> {
        let mut binary = vec![];
        binary.write_i32::<BigEndian>(arity).unwrap();
        binary.extend(elements.iter().clone());
        self.generate_term(BertTag::LargeTuple, binary)
    }

    fn enum_value_to_binary(&self, enum_value: BertType) -> Vec<u8> {
        match enum_value {
            BertType::SmallInteger(value_u8) => self.to_bert(value_u8),
            BertType::Integer(value_i32) => self.to_bert(value_i32),
            BertType::Float(value_f64) => self.to_bert(value_f64),
            BertType::String(string) => self.to_bert(string),
            BertType::Boolean(boolean) => self.to_bert(boolean),
            BertType::Tuple(tuple) => self.to_bert(tuple),
            BertType::Atom(atom_name) => self.get_atom(&atom_name)
        }
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
        self.generate_term(BertTag::NewFloat, binary)
    }
}


impl Serialize<String> for Serializer {
    fn to_bert(&self, data: String) -> Vec<u8> {
        let binary_string = self.convert_string_to_binary(&data);
        self.generate_term(BertTag::String, binary_string)
    }
}


impl<'a> Serialize<&'a str> for Serializer {
    fn to_bert(&self, data: &'a str) -> Vec<u8> {
        let binary_string = self.convert_string_to_binary(data);
        self.generate_term(BertTag::String, binary_string)
    }
}


impl Serialize<bool> for Serializer {
    fn to_bert(&self, data: bool) -> Vec<u8> {
        let boolean_string = data.to_string();

        let bert_atom = self.get_bert_atom();
        let boolean_atom = self.get_atom(&boolean_string);

        let binary = self.merge_terms(bert_atom, boolean_atom);
        self.get_small_tuple(2, binary)
    }
}


impl Serialize<BertTuple> for Serializer {
    fn to_bert(&self, data: BertTuple) -> Vec<u8> {
        let arity = data.values.len();
        let binary: Vec<u8> = data.values
            .into_iter()
            .flat_map(|item| self.enum_value_to_binary(item).into_iter())
            .collect();

        match arity {
            0...255 => self.get_small_tuple(arity as u8, binary),
            _ => self.get_large_tuple(arity as i32, binary),
        }
    }
}


#[cfg(test)]
mod test_serializer {
    use super::{Serializer};
    use types::{BertTag, BertTuple, BertType};

    use byteorder::{BigEndian, WriteBytesExt};

    #[test]
    fn test_generate_term() {
        let serializer = Serializer::new();

        let data: Vec<u8> = vec![0, 4, 116, 101, 115, 116];  // bert as string
        assert_eq!(
            serializer.generate_term(BertTag::Atom, data),
            vec![100u8, 0, 4, 116, 101, 115, 116]
        );
    }

    #[test]
    fn test_merge_terms() {
        let serializer = Serializer::new();

        let term_1: Vec<u8> = vec![100, 0, 4,  98, 101, 114, 116];
        let term_2: Vec<u8> = vec![100, 0, 3, 110, 105, 108];
        assert_eq!(
            serializer.merge_terms(term_1, term_2),
            vec![
                100u8, 0, 4, 98, 101, 114, 116,  // "bert" as atom
                100,   0, 3, 110, 105, 108       // "nil" as atom
            ]
        );
    }

    #[test]
    fn test_convert_string_to_binary() {
        let serializer = Serializer::new();

        let data = "bert";
        assert_eq!(
            serializer.convert_string_to_binary(data),
            vec![0u8, 4, 98, 101, 114, 116]
        );
    }

    #[test]
    fn test_get_atom() {
        let serializer = Serializer::new();

        assert_eq!(
            serializer.get_atom("test"),
            vec![100u8, 0, 4, 116, 101, 115, 116]
        );
    }

    #[test]
    fn test_get_nil() {
        let serializer = Serializer::new();

        assert_eq!(
            serializer.get_nil(),
            vec![106u8]
        );
    }

    #[test]
    fn test_get_bert_nil() {
        let serializer = Serializer::new();

        assert_eq!(
            serializer.get_bert_nil(),
            vec![
                104u8,
                2,                              // tuple length
                100, 0, 4,  98, 101, 114, 116,  // "bert" as atom
                100, 0, 3, 110, 105, 108        // "nil" as atom
            ]
        )
    }

    #[test]
    fn test_serialize_u8() {
        let serializer = Serializer::new();

        assert_eq!(
            serializer.term_to_binary(1u8),
            vec![131u8, 97, 1]
        );

        assert_eq!(
            serializer.term_to_binary(255u8),
            vec![131u8, 97, 255]
        );
    }

    #[test]
    fn test_serialize_i32() {
        let serializer = Serializer::new();

        assert_eq!(
            serializer.term_to_binary(-2147483648),
            vec![131u8, 98, 128, 0, 0, 0]
        );

        assert_eq!(
            serializer.term_to_binary(-1i32),
            vec![131u8, 98, 255, 255, 255, 255]
        );

        assert_eq!(
            serializer.term_to_binary(512i32),
            vec![131u8, 98, 0, 0, 2, 0]
        );

        assert_eq!(
            serializer.term_to_binary(2147483647),
            vec![131u8, 98, 127, 255, 255, 255]
        );
    }

    #[test]
    fn test_serializer_f64() {
        let serializer = Serializer::new();

        assert_eq!(
            serializer.term_to_binary(-3.14f64),
            vec![131u8, 70, 192, 9, 30, 184, 81, 235, 133, 31]
        );

        assert_eq!(
            serializer.term_to_binary(0.0f64),
            vec![131u8, 70, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        assert_eq!(
            serializer.term_to_binary(3.14f64),
            vec![131u8, 70, 64, 9, 30, 184, 81, 235, 133, 31]
        )

    }

    #[test]
    fn test_serialize_bool() {
        let serializer = Serializer::new();

        assert_eq!(
            serializer.term_to_binary(true),
            vec![
                131u8,
                104,                            // small tuple tag
                2,                              // tuple length
                100, 0, 4,  98, 101, 114, 116,  // "bert" as atom
                100, 0, 4, 116, 114, 117, 101   // "true" as atom
            ]
        );

        assert_eq!(
            serializer.term_to_binary(false),
            vec![
                131u8,
                104,                               // small tuple tag
                2,                                 // tuple length
                100, 0, 4, 98, 101, 114, 116,      // "bert" as atom
                100, 0, 5, 102, 97, 108, 115, 101  // "false" as atom
            ]
        );
    }

    #[test]
    fn test_serialize_string() {
        let serializer = Serializer::new();

        // string by value
        assert_eq!(
            serializer.term_to_binary(String::from("test")),
            vec![131u8, 107, 0, 4, 116, 101, 115, 116]
        );

        // string by reference
        assert_eq!(
            serializer.term_to_binary("test"),
            vec![131u8, 107, 0, 4, 116, 101, 115, 116]
        );
    }

    #[test]
    fn test_serializer_tuples() {
        let serializer = Serializer::new();

        let mut tuple_size: i32;
        let mut data_sample: Vec<BertType>;
        let mut serialized_data: Vec<u8>;

        // small tuple
        data_sample = vec![
            BertType::SmallInteger(1u8), BertType::Integer(4i32),
            BertType::Float(8.1516f64), BertType::String(String::from("test")),
            BertType::Atom(String::from("value"))
        ];
        assert_eq!(
            serializer.term_to_binary(BertTuple{values: data_sample}),
            vec![131u8,
                 104,                                      // tuple
                 5,                                        // length
                 97, 1,                                    // 1 as u8
                 98, 0, 0, 0, 4,                           // 4 as i32
                 70, 64, 32, 77, 158, 131, 228, 37, 175,   // 8.1516 as f64
                 107, 0, 4, 116, 101, 115, 116,            // "test" as string
                 100, 0, 5, 118, 97, 108, 117, 101         // "value as atom
            ]
        );

        // small tuple with max capacity
        tuple_size = 255;
        data_sample = vec![];
        serialized_data = vec![131u8, 104, tuple_size as u8];
        for _ in 0..tuple_size {
            data_sample.push(BertType::SmallInteger(1u8));

            serialized_data.push(97); // 97 is ID for u8 type in BERT
            serialized_data.push(1);  // value
        }
        assert_eq!(
            serializer.term_to_binary(BertTuple{values: data_sample}),
            serialized_data
        );

        // large_tuple
        tuple_size = 512;
        data_sample = vec![];
        serialized_data = vec![131u8, 105];
        serialized_data.write_i32::<BigEndian>(tuple_size).unwrap();
        for _ in 0..tuple_size {
            data_sample.push(BertType::SmallInteger(1u8));

            serialized_data.push(97); // 97 is ID for u8 type in BERT
            serialized_data.push(1);  // value
        }
        assert_eq!(
            serializer.term_to_binary(BertTuple{values: data_sample}),
            serialized_data
        );
    }
}
