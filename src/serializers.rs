use std::io;
use std::string::String;
use std::vec::Vec;

use byteorder::{BigEndian, WriteBytesExt};
use num::bigint::{BigInt, Sign};
use serde::ser;

use errors::{Error, Result};
use types::{BERT_LABEL, EXT_VERSION, BertTag};


// TODO: Add support for struct, variants, map

pub enum State {
    Empty,
    First,
    Rest,
}


pub struct Serializer<W>{
    writer: W
}


pub trait Convert<T> {
    fn to_binary(&self, data: T) -> Vec<u8>;
}


impl<W> Serializer<W> where W: io::Write, {
    pub fn new(writer: W) -> Serializer<W> {
        Serializer{writer: writer}
    }

    /// Unwrap the `Writer` from the `Serializer`.
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }

    pub fn generate_term(&mut self, tag: BertTag, data: Vec<u8>) -> Result<()> {
        let header = vec![tag as u8];
        let binary = self.merge_terms(header, data);
        self.writer.write_all(binary.as_slice()).map_err(From::from)
    }

    pub fn merge_terms(&self, term_1: Vec<u8>, term_2: Vec<u8>) -> Vec<u8> {
        let mut binary = term_1.clone();
        binary.extend(term_2.iter().clone());
        binary
    }

    pub fn get_atom(&self, name: &str) -> Vec<u8> {
        let header = vec![BertTag::Atom as u8];
        let name = self.to_binary(name);
        self.merge_terms(header, name)
    }

    pub fn get_nil(&self) -> Vec<u8> {
        vec![BertTag::Nil as u8]
    }

    pub fn get_bert_nil(&self) -> Vec<u8> {
        let bert_atom = self.get_bert_atom();
        let nil_atom = self.get_atom("nil");

        let mut binary = vec![];
        binary.extend(bert_atom.iter().clone());
        binary.extend(nil_atom.iter().clone());
        self.get_small_tuple(2, binary)
    }

    pub fn get_bert_atom(&self) -> Vec<u8> {
        self.get_atom(BERT_LABEL)
    }

    fn get_empty_tuple(&self) -> Vec<u8> {
        vec![BertTag::SmallTuple as u8, 0]
    }

    fn get_small_tuple(&self, arity: u8, elements: Vec<u8>) -> Vec<u8> {
        let header = vec![BertTag::SmallTuple as u8, arity];
        self.merge_terms(header, elements)
    }

//    fn get_big_number_sign(&self, sign: Sign) -> u8 {
//        match sign {
//            Sign::Plus => 0,
//            Sign::Minus => 1,
//            _ => panic!("Invalid bignum sign.")
//        }
//    }
//
//    fn get_small_big_number(&self, sign: Sign, length: u8, bytes: Vec<u8>) -> Vec<u8> {
//        let byte_sign = self.get_big_number_sign(sign);
//        let mut binary = vec![BertTag::BigNum as u8, length];
//        binary.write_u8(byte_sign).unwrap();
//        binary.extend(bytes.iter().clone());
//        binary
//    }
//
//    fn get_large_big_number(&self, sign: Sign, length: u32, bytes: Vec<u8>) -> Vec<u8> {
//        let byte_sign = self.get_big_number_sign(sign);
//        let mut binary = vec![BertTag::LargeNum as u8];
//        binary.write_u32::<BigEndian>(length).unwrap();
//        binary.write_u8(byte_sign).unwrap();
//        binary.extend(bytes.iter().clone());
//        binary
//    }
}


impl<'a, W> Convert<&'a str> for Serializer<W> where W: io::Write {
    fn to_binary(&self, data: &'a str) -> Vec<u8> {
        let binary_string = data.as_bytes();
        let binary_length = binary_string.len() as i16;
        let mut binary = vec![];
        binary.write_i16::<BigEndian>(binary_length).unwrap();
        binary.extend(binary_string.iter().clone());
        binary
    }
}


impl<W> ser::Serializer for Serializer<W> where W: io::Write {
    type Error = Error;

    type SeqState = State;
    type TupleState = State;
    type TupleStructState = State;
    type TupleVariantState = State;
    type MapState = State;
    type StructState = State;
    type StructVariantState = State;

    #[inline]
    fn serialize_bool(&mut self, value: bool) -> Result<()> {
        let boolean_string = value.to_string();

        let bert_atom = self.get_bert_atom();
        let boolean_atom = self.get_atom(&boolean_string);

        let binary = self.merge_terms(bert_atom, boolean_atom);
        let tuple = self.get_small_tuple(2, binary);
        self.writer.write_all(tuple.as_slice()).map_err(From::from)
    }

    #[inline]
    fn serialize_isize(&mut self, value: isize) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_i8(&mut self, value: i8) -> Result<()> {
        self.serialize_i32(value as i32)
    }

    #[inline]
    fn serialize_i16(&mut self, value: i16) -> Result<()> {
        self.serialize_i32(value as i32)
    }

    #[inline]
    fn serialize_i32(&mut self, value: i32) -> Result<()> {
        let mut binary = vec![];
        binary.write_i32::<BigEndian>(value).unwrap();
        self.generate_term(BertTag::Integer, binary);
        Ok(())
    }

    #[inline]
    fn serialize_i64(&mut self, value: i64) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_usize(&mut self, value: usize) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_u8(&mut self, value: u8) -> Result<()> {
        self.generate_term(BertTag::SmallInteger, vec![value])
    }

    #[inline]
    fn serialize_u16(&mut self, value: u16) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_u32(&mut self, value: u32) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_u64(&mut self, value: u64) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_f32(&mut self, value: f32) -> Result<()> {
        self.serialize_f64(value as f64)
    }

    #[inline]
    fn serialize_f64(&mut self, value: f64) -> Result<()> {
        let mut binary = vec![];
        binary.write_f64::<BigEndian>(value).unwrap();
        self.generate_term(BertTag::NewFloat, binary)
    }

    #[inline]
    fn serialize_char(&mut self, value: char) -> Result<()> {
        self.serialize_str(&value.to_string())
    }

    #[inline]
    fn serialize_str(&mut self, value: &str) -> Result<()> {
        let binary_string = self.to_binary(value);
        self.generate_term(BertTag::String, binary_string)
    }

    #[inline]
    fn serialize_bytes(&mut self, data: &[u8]) -> Result<()> {
        let length = data.len();

        let mut header = vec![BertTag::Binary as u8];
        header.write_i32::<BigEndian>(length as i32).unwrap();
        try!(self.writer.write_all(header.as_slice()));

        for byte in data {
            try!(self.writer.write_all(&[*byte]))
        };

        self.serialize_seq_end(State::Empty)
    }

    #[inline]
    fn serialize_unit(&mut self) -> Result<()> {
        let nil = self.get_nil();
        self.writer.write_all(nil.as_slice()).map_err(From::from)
    }

    #[inline]
    fn serialize_unit_struct(&mut self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        &mut self, _name: &'static str, _variant_index: usize,
        variant: &'static str
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    /// Serialize newtypes without an object wrapper
    #[inline]
    fn serialize_newtype_struct<T>(
        &mut self, _name: &'static str, value: T
    ) -> Result<()> where T: ser::Serialize {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        &mut self, _name: &'static str, _variant_index: usize,
        variant: &'static str, value: T
    ) -> Result<()> where T: ser::Serialize {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_none(&mut self) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_some<T>(
        &mut self, value: T
    ) -> Result<()> where T: ser::Serialize {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_seq(&mut self, len: Option<usize>) -> Result<State> {
        match len {
            Some(0) => {
                let bert_nil_tuple = self.get_bert_nil();
                try!(self.writer.write_all(bert_nil_tuple.as_slice()));
                Ok(State::Empty)
            },
            Some(list_length) => {
                let mut header = vec![BertTag::List as u8];
                header.write_i32::<BigEndian>(list_length as i32).unwrap();
                try!(self.writer.write_all(header.as_slice()));
                Ok(State::First)
            }
            None => Ok(State::Empty)
        }
    }

    #[inline]
    fn serialize_seq_elt<T: ser::Serialize>(
        &mut self, state: &mut State, value: T
    ) -> Result<()> where T: ser::Serialize {
        *state = State::Rest;
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq_end(&mut self, state: State) -> Result<()> {
        match state {
            State::Empty => Ok(()),
            _ =>  {
                let nil = self.get_nil();
                try!(self.writer.write_all(nil.as_slice()));
                Ok(())
            }
        }
    }

    #[inline]
    fn serialize_seq_fixed_size(&mut self, size: usize) -> Result<State> {
        self.serialize_seq(Some(size))
    }

    #[inline]
    fn serialize_tuple(&mut self, len: usize) -> Result<State> {
        match len {
            0 => {
                let empty_tuple = self.get_empty_tuple();
                try!(self.writer.write_all(empty_tuple.as_slice()));
                Ok(State::Empty)
            },
            1...255 => {
                let header = vec![BertTag::SmallTuple as u8, len as u8];
                try!(self.writer.write_all(header.as_slice()));
                Ok(State::First)
            }
            _ => {
                let mut header = vec![BertTag::LargeTuple as u8];
                header.write_i32::<BigEndian>(len as i32).unwrap();
                try!(self.writer.write_all(header.as_slice()));
                Ok(State::First)
            }
        }
    }

    #[inline]
    fn serialize_tuple_elt<T: ser::Serialize>(
        &mut self, state: &mut State, value: T
    ) -> Result<()> {
        *state = State::Rest;
        value.serialize(self)
    }

    #[inline]
    fn serialize_tuple_end(&mut self, state: State) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_tuple_struct(
        &mut self, _name: &'static str, len: usize
    ) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_struct_elt<T: ser::Serialize>(
        &mut self, state: &mut State, value: T
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_struct_end(&mut self, state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_variant(
        &mut self, _name: &'static str, _variant_index: usize,
        variant: &'static str, len: usize
    ) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_variant_elt<T: ser::Serialize>(
        &mut self, state: &mut State, value: T
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_variant_end(&mut self, state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_map(&mut self, len: Option<usize>) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_map_key<T: ser::Serialize>(
        &mut self, state: &mut State, key: T,
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_map_value<T: ser::Serialize>(
        &mut self, _: &mut State, value: T
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_map_end(&mut self, state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct(
        &mut self, _name: &'static str, len: usize
    ) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct_elt<V: ser::Serialize>(
        &mut self, state: &mut State, key: &'static str, value: V
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct_end(&mut self, state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct_variant(
        &mut self, _name: &'static str, _variant_index: usize,
        variant: &'static str, len: usize
    ) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct_variant_elt<V: ser::Serialize>(
        &mut self, state: &mut State, key: &'static str, value: V
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct_variant_end(&mut self, state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }
}


/// Encode the passed value into a `[u8]` writer.
#[inline]
pub fn to_writer<W, T>(
        writer: &mut W, value: &T
    ) -> Result<()> where W: io::Write, T: ser::Serialize {
    let mut ser = Serializer::new(writer);
    try!(value.serialize(&mut ser));
    Ok(())
}


/// Encode the specified struct into a `[u8]` buffer.
#[inline]
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>> where T: ser::Serialize {
    let mut writer = Vec::with_capacity(128);
    try!(to_writer(&mut writer, value));
    Ok(writer)
}


/// Convert passed value to a BERT representation.
#[inline]
pub fn term_to_binary<T> (
        value: &T
    ) -> Result<Vec<u8>> where T: ser::Serialize {
    let mut binary = vec![EXT_VERSION];
    let data = try!(to_vec(value));
    binary.extend(data.iter().clone());
    Ok(binary)
}


#[cfg(test)]
mod test_serializer {
    use serde::bytes::{Bytes};

    use super::{Serializer, term_to_binary, to_vec};
    use types::{BertTag};

    #[test]
    fn test_generate_term() {
        let mut writer = Vec::with_capacity(128);
        let mut bert = Serializer::new(&mut writer);

        let data: Vec<u8> = vec![0, 4, 116, 101, 115, 116];  // bert as string
        bert.generate_term(BertTag::Atom, data);
        assert_eq!(
            *bert.into_inner(),
            vec![100u8, 0, 4, 116, 101, 115, 116]
        );
    }

    #[test]
    fn test_merge_terms() {
        let mut writer = Vec::with_capacity(128);
        let mut bert = Serializer::new(&mut writer);

        let term_1: Vec<u8> = vec![100, 0, 4,  98, 101, 114, 116];
        let term_2: Vec<u8> = vec![100, 0, 3, 110, 105, 108];
        assert_eq!(
            bert.merge_terms(term_1, term_2),
            vec![
                100u8, 0, 4, 98, 101, 114, 116,  // "bert" as atom
                100,   0, 3, 110, 105, 108       // "nil" as atom
            ]
        );
    }

    #[test]
    fn test_get_atom() {
        let mut writer = Vec::with_capacity(128);
        let mut bert = Serializer::new(&mut writer);

        assert_eq!(
            bert.get_atom("test"),
            vec![100u8, 0, 4, 116, 101, 115, 116]
        );
    }

    #[test]
    fn test_get_nil() {
        let mut writer = Vec::with_capacity(128);
        let mut bert = Serializer::new(&mut writer);

        assert_eq!(
            bert.get_nil(),
            vec![106u8]
        );
    }

    #[test]
    fn test_get_bert_nil() {
        let mut writer = Vec::with_capacity(128);
        let mut bert = Serializer::new(&mut writer);

        assert_eq!(
            bert.get_bert_nil(),
            vec![
                104u8,
                2,                              // tuple length
                100, 0, 4,  98, 101, 114, 116,  // "bert" as atom
                100, 0, 3, 110, 105, 108        // "nil" as atom
            ]
        );
    }

    #[test]
    fn test_get_bert_atom() {
        let mut writer = Vec::with_capacity(128);
        let mut bert = Serializer::new(&mut writer);

        assert_eq!(
            bert.get_bert_atom(),
            vec![100, 0, 4,  98, 101, 114, 116] // "bert" as atom
        );
    }

    #[test]
    fn test_serialize_bool() {
        assert_eq!(
            term_to_binary(&true).unwrap(),
            vec![
                131u8,
                104,                            // small tuple tag
                2,                              // tuple length
                100, 0, 4,  98, 101, 114, 116,  // "bert" as atom
                100, 0, 4, 116, 114, 117, 101   // "true" as atom
            ]
        );

        assert_eq!(
            term_to_binary(&false).unwrap(),
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
    #[should_panic]
    fn test_serialize_isize() {
        let value: isize = 100;
        term_to_binary(&value).unwrap();
    }

    #[test]
    fn test_serialize_i8() {
        assert_eq!(
            term_to_binary(&-128i8).unwrap(),
            vec![131u8, 98, 255, 255, 255, 128]
        );

        assert_eq!(
            term_to_binary(&-1i8).unwrap(),
            vec![131u8, 98, 255, 255, 255, 255]
        );

        assert_eq!(
            term_to_binary(&64i8).unwrap(),
            vec![131u8, 98, 0, 0, 0, 64]
        );

        assert_eq!(
            term_to_binary(&127i8).unwrap(),
            vec![131u8, 98, 0, 0, 0, 127]
        );
    }

    #[test]
    fn test_serialize_i16() {
        assert_eq!(
            term_to_binary(&-32768i16).unwrap(),
            vec![131u8, 98, 255, 255, 128, 0]
        );

        assert_eq!(
            term_to_binary(&-1i16).unwrap(),
            vec![131u8, 98, 255, 255, 255, 255]
        );

        assert_eq!(
            term_to_binary(&512i16).unwrap(),
            vec![131u8, 98, 0, 0, 2, 0]
        );

        assert_eq!(
            term_to_binary(&32767i16).unwrap(),
            vec![131u8, 98, 0, 0, 127, 255]
        );
    }

    #[test]
    fn test_serialize_i32() {
        assert_eq!(
            term_to_binary(&-2147483648i32).unwrap(),
            vec![131u8, 98, 128, 0, 0, 0]
        );

        assert_eq!(
            term_to_binary(&-1i32).unwrap(),
            vec![131u8, 98, 255, 255, 255, 255]
        );

        assert_eq!(
            term_to_binary(&512i32).unwrap(),
            vec![131u8, 98, 0, 0, 2, 0]
        );

        assert_eq!(
            term_to_binary(&2147483647i32).unwrap(),
            vec![131u8, 98, 127, 255, 255, 255]
        );
    }

    #[test]
    #[should_panic]
    fn test_serialize_i64() {
        term_to_binary(&1000i64).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_serialize_usize() {
        let value: usize = 100;
        term_to_binary(&value).unwrap();
    }

    #[test]
    fn test_serialize_u8() {
        assert_eq!(
            term_to_binary(&1u8).unwrap(),
            vec![131u8, 97, 1]
        );

        assert_eq!(
            term_to_binary(&255u8).unwrap(),
            vec![131u8, 97, 255]
        );
    }

    #[test]
    #[should_panic]
    fn test_serialize_u16() {
        term_to_binary(&100u16).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_serialize_u32() {
        term_to_binary(&100u32).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_serialize_u64() {
        term_to_binary(&100u64).unwrap();
    }

    #[test]
    fn test_serialize_f32() {
        assert_eq!(
            term_to_binary(&-3.14f32).unwrap(),
            vec![131u8, 70, 192, 9, 30, 184, 96, 0, 0, 0]
        );

        assert_eq!(
            term_to_binary(&0.0f32).unwrap(),
            vec![131u8, 70, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        assert_eq!(
            term_to_binary(&3.14f32).unwrap(),
            vec![131u8, 70, 64, 9, 30, 184, 96, 0, 0, 0]
        );
    }

    #[test]
    fn test_serialize_f64() {
        assert_eq!(
            term_to_binary(&-3.14f64).unwrap(),
            vec![131u8, 70, 192, 9, 30, 184, 81, 235, 133, 31]
        );

        assert_eq!(
            term_to_binary(&0.0f64).unwrap(),
            vec![131u8, 70, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        assert_eq!(
            term_to_binary(&3.14f64).unwrap(),
            vec![131u8, 70, 64, 9, 30, 184, 81, 235, 133, 31]
        );
    }

    #[test]
    fn test_serialize_char() {
        assert_eq!(
            term_to_binary(&'a').unwrap(),
            vec![131u8, 107, 0, 1, 97]
        );
    }

    #[test]
    fn test_serialize_string() {
        assert_eq!(
            term_to_binary(&"test").unwrap(),
            vec![131u8, 107, 0, 4, 116, 101, 115, 116]
        );
    }

    // TODO: Fix the tests after adding specialization support in Rust
    #[test]
    fn test_serialize_bytes() {
        let empty_bytes_list: Bytes = b""[..].into();

        assert_eq!(
            term_to_binary(&empty_bytes_list).unwrap(),
            vec![
                131u8,
                109,         // binary
                0, 0, 0, 0   // length
            ]
        );

        let bytes_array: Bytes = b"value"[..].into();

        assert_eq!(
            term_to_binary(&bytes_array).unwrap(),
            vec![
                131u8,
                109,         // binary
                0, 0, 0, 5,  // length
                118,         // "v"
                97,          // "a"
                108,         // "l"
                117,         // "u"
                101          // "e"
            ]
        );
    }

    #[test]
    fn test_serialize_tuple() {
        let small_tuple = (1u8, 4i32, 8.1516f64, String::from("value"));

        assert_eq!(
            term_to_binary(&small_tuple).unwrap(),
            vec![
                131u8,
                104,                                    // tuple
                4,                                      // length
                97, 1,                                  // 1
                98, 0, 0, 0, 4,                         // 4
                70, 64, 32, 77, 158, 131, 228, 37, 175, // 8.1516
                107, 0, 5, 118, 97, 108, 117, 101       // "value" as string
            ]
        );
    }

    #[test]
    fn test_serialize_list() {
        let empty_list: Vec<i32> = vec![];

        assert_eq!(
            term_to_binary(&empty_list).unwrap(),
            vec![
                131u8,
                104,                            // tuple
                2,                              // tuple length
                100, 0, 4,  98, 101, 114, 116,  // "bert" as atom
                100, 0, 3, 110, 105, 108        // "nil" as atom
            ]
        );

        let list = [1i32, 2, 3];

        assert_eq!(
            term_to_binary(&list).unwrap(),
            vec![
                131u8,
                108,                            // list
                0, 0, 0, 3,                     // length
                98, 0, 0, 0, 1,                 // 1
                98, 0, 0, 0, 2,                 // 2
                98, 0, 0, 0, 3,                 // 3
                106                             // "nil" as atom
            ]
        );
    }
}
