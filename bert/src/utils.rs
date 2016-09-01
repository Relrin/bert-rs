use byteorder::{BigEndian, WriteBytesExt};

use types::{BERT_LABEL, BertTag};


pub fn merge_terms(term_1: Vec<u8>, term_2: Vec<u8>) -> Vec<u8> {
    let mut binary = term_1.clone();
    binary.extend(term_2.iter().clone());
    binary
}


pub fn str_to_binary(data: &str) -> Vec<u8> {
    let binary_string = data.as_bytes();
    let binary_length = binary_string.len() as i16;
    let mut binary = vec![];
    binary.write_i16::<BigEndian>(binary_length).unwrap();
    binary.extend(binary_string.iter().clone());
    binary
}


pub fn get_atom(name: &str) -> Vec<u8> {
    let header = vec![BertTag::Atom as u8];
    let normalized_name = &name.to_string().to_lowercase();
    let name = str_to_binary(normalized_name);
    merge_terms(header, name)
}


pub fn get_nil() -> Vec<u8> {
    vec![BertTag::Nil as u8]
}


pub fn get_bert_nil() -> Vec<u8> {
    let bert_atom = get_bert_atom();
    let nil_atom = get_atom("nil");

    let mut binary = vec![];
    binary.extend(bert_atom.iter().clone());
    binary.extend(nil_atom.iter().clone());
    get_small_tuple(2, binary)
}


pub fn get_bert_atom() -> Vec<u8> {
    get_atom(BERT_LABEL)
}


pub fn get_empty_tuple() -> Vec<u8> {
    vec![BertTag::SmallTuple as u8, 0]
}


pub fn get_small_tuple(arity: u8, elements: Vec<u8>) -> Vec<u8> {
    let header = vec![BertTag::SmallTuple as u8, arity];
    merge_terms(header, elements)
}
