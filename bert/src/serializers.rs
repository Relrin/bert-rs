use std::io;
use std::string::String;
use std::vec::Vec;

use byteorder::{BigEndian, WriteBytesExt};
use num::bigint::{BigInt, Sign};
use serde::ser;

use errors::{Error, Result};
use types::{BERT_LABEL, EXT_VERSION, BertTag};


// TODO: Add support for map, BertTime, BertRegex

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
        let normalized_name = &name.to_string().to_lowercase();
        let name = self.to_binary(normalized_name);
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
        self.serialize_i32(value as i32)
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

    #[inline]
    fn serialize_newtype_struct<T>(
        &mut self, _name: &'static str, value: T
    ) -> Result<()> where T: ser::Serialize {
        let header = vec![BertTag::SmallTuple as u8, 2u8];
        try!(self.writer.write_all(header.as_slice()));

        let structure_name_atom = self.get_atom(_name);
        try!(self.writer.write_all(structure_name_atom.as_slice()));

        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        &mut self, _name: &'static str, _variant_index: usize,
        variant: &'static str, value: T
    ) -> Result<()> where T: ser::Serialize {
        let header = vec![BertTag::SmallTuple as u8, 2u8];
        try!(self.writer.write_all(header.as_slice()));

        let enum_atom = self.get_atom(_name);
        try!(self.writer.write_all(enum_atom.as_slice()));

        let variant_header = vec![BertTag::SmallTuple as u8, 2u8];
        try!(self.writer.write_all(variant_header.as_slice()));

        let variant_atom = self.get_atom(variant);
        try!(self.writer.write_all(variant_atom.as_slice()));

        value.serialize(self)
    }

    #[inline]
    fn serialize_none(&mut self) -> Result<()> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T>(
        &mut self, value: T
    ) -> Result<()> where T: ser::Serialize {
        value.serialize(self)
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
        let tuple_size = len + 1; // include name of entity
        let mut header = vec![BertTag::LargeTuple as u8];
        header.write_i32::<BigEndian>(tuple_size as i32).unwrap();
        try!(self.writer.write_all(header.as_slice()));

        let structure_name = self.get_atom(_name);
        try!(self.writer.write_all(structure_name.as_slice()));

        Ok(State::First)
    }

    #[inline]
    fn serialize_tuple_struct_elt<T: ser::Serialize>(
        &mut self, state: &mut State, value: T
    ) -> Result<()> {
        *state = State::Rest;
        value.serialize(self)
    }

    #[inline]
    fn serialize_tuple_struct_end(&mut self, state: State) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_tuple_variant(
        &mut self, _name: &'static str, _variant_index: usize,
        variant: &'static str, len: usize
    ) -> Result<State> {
        let mut header = vec![BertTag::LargeTuple as u8];
        header.write_i32::<BigEndian>(2i32).unwrap();
        try!(self.writer.write_all(header.as_slice()));

        let enum_name = self.get_atom(_name);
        try!(self.writer.write_all(enum_name.as_slice()));

        let mut variant_header = vec![BertTag::LargeTuple as u8];
        let variant_length = len as i32 + 1; // include variant name also
        variant_header.write_i32::<BigEndian>(variant_length).unwrap();
        try!(self.writer.write_all(variant_header.as_slice()));

        let variant_name = self.get_atom(variant);
        try!(self.writer.write_all(variant_name.as_slice()));

        Ok(State::First)
    }

    #[inline]
    fn serialize_tuple_variant_elt<T: ser::Serialize>(
        &mut self, state: &mut State, value: T
    ) -> Result<()> {
        *state = State::Rest;
        value.serialize(self)
    }

    #[inline]
    fn serialize_tuple_variant_end(&mut self, state: State) -> Result<()> {
        Ok(())
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
        let mut header = vec![BertTag::LargeTuple as u8];
        let tuple_length = len as i32 + 1; // include name of structure
        header.write_i32::<BigEndian>(tuple_length).unwrap();
        try!(self.writer.write_all(header.as_slice()));

        let structure_name_atom = self.get_atom(_name);
        try!(self.writer.write_all(structure_name_atom.as_slice()));

        Ok(State::First)
    }

    #[inline]
    fn serialize_struct_elt<V: ser::Serialize>(
        &mut self, state: &mut State, key: &'static str, value: V
    ) -> Result<()> {
        *state = State::Rest;

        let header = vec![BertTag::SmallTuple as u8, 2u8];
        try!(self.writer.write_all(header.as_slice()));

        let field_atom = self.get_atom(key);
        try!(self.writer.write_all(field_atom.as_slice()));

        value.serialize(self)
    }

    #[inline]
    fn serialize_struct_end(&mut self, state: State) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_struct_variant(
        &mut self, _name: &'static str, _variant_index: usize,
        variant: &'static str, len: usize
    ) -> Result<State> {
        let mut header = vec![BertTag::LargeTuple as u8];
        header.write_i32::<BigEndian>(2i32).unwrap();
        try!(self.writer.write_all(header.as_slice()));

        let enum_name = self.get_atom(_name);
        try!(self.writer.write_all(enum_name.as_slice()));

        let mut variant_header = vec![BertTag::LargeTuple as u8];
        let variant_length = len as i32 + 1; // include variant name also
        variant_header.write_i32::<BigEndian>(variant_length).unwrap();
        try!(self.writer.write_all(variant_header.as_slice()));

        let variant_name = self.get_atom(variant);
        try!(self.writer.write_all(variant_name.as_slice()));

        Ok(State::First)
    }

    #[inline]
    fn serialize_struct_variant_elt<V: ser::Serialize>(
        &mut self, state: &mut State, key: &'static str, value: V
    ) -> Result<()> {
        *state = State::Rest;

        let header = vec![BertTag::SmallTuple as u8, 2u8];
        try!(self.writer.write_all(header.as_slice()));

        let field_atom = self.get_atom(key);
        try!(self.writer.write_all(field_atom.as_slice()));

        value.serialize(self)
    }

    #[inline]
    fn serialize_struct_variant_end(&mut self, state: State) -> Result<()> {
        Ok(())
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
