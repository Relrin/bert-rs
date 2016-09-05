//! BERT serialization
//!
//! This module provides for BERT serialization with the type `Serializer`.
use std::io;
use std::vec::Vec;

use byteorder::{BigEndian, WriteBytesExt};
use serde::ser;

use super::errors::{Error, Result};
use super::types::{ETF_VERSION, BertTag};
use super::utils::{
    merge_terms, str_to_binary, get_atom, get_nil, get_bert_nil,
    get_bert_atom, get_empty_tuple, get_small_tuple
};
use super::wrappers::{
    BIGNUM_STRUCT_NAME, TIME_STRUCT_NAME, REGEX_STRUCT_NAME,
    REGEX_OPTION_ENUM_NAME
};


#[doc(hidden)]
#[derive(Eq, PartialEq)]
pub enum State {
    Empty,
    First,
    Rest,
}


// Serializer for the num::BigInt type. Only for internal use.
struct BigNumSerializer<W>{
    writer: W
}


impl<W> BigNumSerializer<W> where W: io::Write, {
    pub fn new(writer: W) -> BigNumSerializer<W> {
        BigNumSerializer{ writer: writer }
    }
}


impl<W> ser::Serializer for BigNumSerializer<W> where W: io::Write {
    type Error = Error;

    type SeqState = State;
    type TupleState = State;
    type TupleStructState = State;
    type TupleVariantState = State;
    type MapState = State;
    type StructState = State;
    type StructVariantState = State;

    #[inline]
    fn serialize_bool(&mut self, _value: bool) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_isize(&mut self, _value: isize) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_i8(&mut self, _value: i8) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_i16(&mut self, _value: i16) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_i32(&mut self, _value: i32) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_i64(&mut self, _value: i64) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_usize(&mut self, _value: usize) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_u8(&mut self, _value: u8) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_u16(&mut self, _value: u16) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_u32(&mut self, _value: u32) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_u64(&mut self, _value: u64) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_f32(&mut self, _value: f32) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_f64(&mut self, _value: f64) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_char(&mut self, _value: char) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_str(&mut self, _value: &str) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_bytes(&mut self, data: &[u8]) -> Result<()> {
        for byte in data {
            try!(self.writer.write_all(&[*byte]))
        };
        Ok(())
    }

    #[inline]
    fn serialize_unit(&mut self) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_unit_struct(&mut self, _name: &'static str) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_unit_variant(
        &mut self, _name: &'static str, _variant_index: usize,
        _variant: &'static str
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_newtype_struct<T>(
        &mut self, _name: &'static str, _value: T
    ) -> Result<()> where T: ser::Serialize {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        &mut self, _name: &'static str, _variant_index: usize,
        _variant: &'static str, _value: T
    ) -> Result<()> where T: ser::Serialize {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_none(&mut self) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_some<T>(
        &mut self, _value: T
    ) -> Result<()> where T: ser::Serialize {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_seq(&mut self, _len: Option<usize>) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_seq_elt<T: ser::Serialize>(
        &mut self, _state: &mut State, _value: T
    ) -> Result<()> where T: ser::Serialize {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_seq_end(&mut self, _state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_seq_fixed_size(&mut self, _size: usize) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple(&mut self, _len: usize) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_elt<T: ser::Serialize>(
        &mut self, _state: &mut State, _value: T
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_end(&mut self, _state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_struct(
        &mut self, _name: &'static str, _len: usize
    ) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_struct_elt<T: ser::Serialize>(
        &mut self, _state: &mut State, _value: T
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_struct_end(&mut self, _state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_variant(
        &mut self, _name: &'static str, _variant_index: usize,
        _variant: &'static str, _len: usize
    ) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_variant_elt<T: ser::Serialize>(
        &mut self, _state: &mut State, _value: T
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_variant_end(&mut self, _state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_map(&mut self, _len: Option<usize>) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_map_key<T: ser::Serialize>(
        &mut self, _state: &mut State, _key: T,
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_map_value<T: ser::Serialize>(
        &mut self, _: &mut State, _value: T
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_map_end(&mut self, _state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct(
        &mut self, _name: &'static str, _len: usize
    ) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct_elt<V: ser::Serialize>(
        &mut self, _state: &mut State, _key: &'static str, _value: V
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct_end(&mut self, _state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct_variant(
        &mut self, _name: &'static str, _variant_index: usize,
        _variant: &'static str, _len: usize
    ) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct_variant_elt<V: ser::Serialize>(
        &mut self, _state: &mut State, _key: &'static str, _value: V
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct_variant_end(&mut self, _state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }
}


// Serializer for the Vec<RegexOption> type. Only for internal use.
struct RegexOptionSerializer<W>{
    writer: W
}


impl<W> RegexOptionSerializer<W> where W: io::Write, {
    pub fn new(writer: W) -> RegexOptionSerializer<W> {
        RegexOptionSerializer{ writer: writer }
    }
}


impl<W> ser::Serializer for RegexOptionSerializer<W> where W: io::Write {
    type Error = Error;

    type SeqState = State;
    type TupleState = State;
    type TupleStructState = State;
    type TupleVariantState = State;
    type MapState = State;
    type StructState = State;
    type StructVariantState = State;

    #[inline]
    fn serialize_bool(&mut self, _value: bool) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_isize(&mut self, _value: isize) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_i8(&mut self, _value: i8) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_i16(&mut self, _value: i16) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_i32(&mut self, _value: i32) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_i64(&mut self, _value: i64) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_usize(&mut self, _value: usize) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_u8(&mut self, _value: u8) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_u16(&mut self, _value: u16) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_u32(&mut self, _value: u32) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_u64(&mut self, _value: u64) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_f32(&mut self, _value: f32) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_f64(&mut self, _value: f64) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_char(&mut self, _value: char) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_str(&mut self, value: &str) -> Result<()> {
        let value = get_atom(value);
        self.writer.write_all(value.as_slice()).map_err(From::from)
    }

    #[inline]
    fn serialize_bytes(&mut self, _data: &[u8]) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_unit(&mut self) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_unit_struct(&mut self, _name: &'static str) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_unit_variant(
        &mut self, _name: &'static str, _variant_index: usize,
        _variant: &'static str
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_newtype_struct<T>(
        &mut self, _name: &'static str, _value: T
    ) -> Result<()> where T: ser::Serialize {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        &mut self, _name: &'static str, _variant_index: usize,
        _variant: &'static str, _value: T
    ) -> Result<()> where T: ser::Serialize {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_none(&mut self) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_some<T>(
        &mut self, _value: T
    ) -> Result<()> where T: ser::Serialize {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_seq(&mut self, len: Option<usize>) -> Result<State> {
        match len {
            Some(0) => {
                let bert_nil_tuple = get_bert_nil();
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
                let nil = get_nil();
                try!(self.writer.write_all(nil.as_slice()));
                Ok(())
            }
        }
    }

    #[inline]
    fn serialize_seq_fixed_size(&mut self, _size: usize) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple(&mut self, _len: usize) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_elt<T: ser::Serialize>(
        &mut self, _state: &mut State, _value: T
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_end(&mut self, _state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_struct(
        &mut self, _name: &'static str, _len: usize
    ) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_struct_elt<T: ser::Serialize>(
        &mut self, _state: &mut State, _value: T
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_struct_end(&mut self, _state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_variant(
        &mut self, _name: &'static str, _variant_index: usize,
        _variant: &'static str, _len: usize
    ) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_variant_elt<T: ser::Serialize>(
        &mut self, _state: &mut State, _value: T
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_variant_end(&mut self, _state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_map(&mut self, _len: Option<usize>) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_map_key<T: ser::Serialize>(
        &mut self, _state: &mut State, _key: T,
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_map_value<T: ser::Serialize>(
        &mut self, _: &mut State, _value: T
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_map_end(&mut self, _state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct(
        &mut self, _name: &'static str, _len: usize
    ) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct_elt<V: ser::Serialize>(
        &mut self, _state: &mut State, _key: &'static str, _value: V
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct_end(&mut self, _state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct_variant(
        &mut self, _name: &'static str, _variant_index: usize,
        _variant: &'static str, _len: usize
    ) -> Result<State> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct_variant_elt<V: ser::Serialize>(
        &mut self, _state: &mut State, _key: &'static str, _value: V
    ) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct_variant_end(&mut self, _state: State) -> Result<()> {
        Err(Error::UnsupportedType)
    }
}



pub struct Serializer<W>{
    writer: W
}


impl<W> Serializer<W> where W: io::Write, {
    pub fn new(writer: W) -> Serializer<W> {
        Serializer{ writer: writer }
    }

    /// Unwrap the `Writer` from the `Serializer`.
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }

    pub fn generate_term(&mut self, tag: BertTag, data: Vec<u8>) -> Result<()> {
        let header = vec![tag as u8];
        let binary = merge_terms(header, data);
        self.writer.write_all(binary.as_slice()).map_err(From::from)
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

        let bert_atom = get_bert_atom();
        let boolean_atom = get_atom(&boolean_string);

        let binary = merge_terms(bert_atom, boolean_atom);
        let tuple = get_small_tuple(2, binary);
        self.writer.write_all(tuple.as_slice()).map_err(From::from)
    }

    #[inline]
    fn serialize_isize(&mut self, value: isize) -> Result<()> {
        self.serialize_i32(value as i32)
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
        self.generate_term(BertTag::Integer, binary).unwrap();
        Ok(())
    }

    #[inline]
    fn serialize_i64(&mut self, _value: i64) -> Result<()> {
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
    fn serialize_u16(&mut self, _value: u16) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_u32(&mut self, _value: u32) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_u64(&mut self, _value: u64) -> Result<()> {
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
        let binary_string = str_to_binary(value);
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
        let nil = get_nil();
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

    // FIXME: Separate parts of code after adding specialization
    #[inline]
    fn serialize_newtype_struct<T>(
        &mut self, _name: &'static str, value: T
    ) -> Result<()> where T: ser::Serialize {
        match _name {
            BIGNUM_STRUCT_NAME => {
                let mut bignum_serializer = BigNumSerializer::new(
                    &mut self.writer
                );
                value.serialize(&mut bignum_serializer)
            },
            REGEX_OPTION_ENUM_NAME => {
                let mut regex_options_serializer = RegexOptionSerializer::new(
                    &mut self.writer
                );
                value.serialize(&mut regex_options_serializer)
            },
            _ => {
                let header = vec![BertTag::SmallTuple as u8, 2u8];
                try!(self.writer.write_all(header.as_slice()));

                let structure_name_atom = get_atom(_name);
                try!(self.writer.write_all(structure_name_atom.as_slice()));

                value.serialize(self)
            }
        }
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        &mut self, _name: &'static str, _variant_index: usize,
        variant: &'static str, value: T
    ) -> Result<()> where T: ser::Serialize {
        let header = vec![BertTag::SmallTuple as u8, 2u8];
        try!(self.writer.write_all(header.as_slice()));

        let enum_atom = get_atom(_name);
        try!(self.writer.write_all(enum_atom.as_slice()));

        let variant_header = vec![BertTag::SmallTuple as u8, 2u8];
        try!(self.writer.write_all(variant_header.as_slice()));

        let variant_atom = get_atom(variant);
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
                let bert_nil_tuple = get_bert_nil();
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
                let nil = get_nil();
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
                let empty_tuple = get_empty_tuple();
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
    fn serialize_tuple_end(&mut self, _state: State) -> Result<()> {
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

        let structure_name = get_atom(_name);
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
    fn serialize_tuple_struct_end(&mut self, _state: State) -> Result<()> {
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

        let enum_name = get_atom(_name);
        try!(self.writer.write_all(enum_name.as_slice()));

        let mut variant_header = vec![BertTag::LargeTuple as u8];
        let variant_length = len as i32 + 1; // include variant name also
        variant_header.write_i32::<BigEndian>(variant_length).unwrap();
        try!(self.writer.write_all(variant_header.as_slice()));

        let variant_name = get_atom(variant);
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
    fn serialize_tuple_variant_end(&mut self, _state: State) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_map(&mut self, len: Option<usize>) -> Result<State> {
        let header = vec![BertTag::SmallTuple as u8, 3u8];
        let bert_atom = get_bert_atom();
        let dict_atom = get_atom("dict");

        try!(self.writer.write_all(header.as_slice()));
        try!(self.writer.write_all(bert_atom.as_slice()));
        try!(self.writer.write_all(dict_atom.as_slice()));

        let mut list_header: Vec<u8> = vec![];

        let state = match len {
            Some(0) | None => {
                list_header.push(BertTag::Nil as u8);
                Ok(State::Empty)
            },
            Some(length) => {
                list_header.push(BertTag::List as u8);
                list_header.write_i32::<BigEndian>(length as i32).unwrap();
                Ok(State::First)
            }
        };

        try!(self.writer.write_all(list_header.as_slice()));
        state
    }

    #[inline]
    fn serialize_map_key<T: ser::Serialize>(
        &mut self, state: &mut State, key: T,
    ) -> Result<()> {
        *state = State::Rest;

        let tuple_header = vec![BertTag::SmallTuple as u8, 2u8];
        try!(self.writer.write_all(tuple_header.as_slice()));
        key.serialize(self)
    }

    #[inline]
    fn serialize_map_value<T: ser::Serialize>(
        &mut self, _: &mut State, value: T
    ) -> Result<()> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_map_end(&mut self, state: State) -> Result<()> {
        if state == State::Rest {
            let nil_atom = get_nil();
            try!(self.writer.write_all(nil_atom.as_slice()));
        }
        Ok(())
    }

    #[inline]
    fn serialize_struct(
        &mut self, _name: &'static str, len: usize
    ) -> Result<State> {
        match _name {
            TIME_STRUCT_NAME => {
                let header = vec![BertTag::SmallTuple as u8, len as u8];
                let bert_atom = get_bert_atom();
                let time_atom = get_atom("time");
                try!(self.writer.write_all(header.as_slice()));
                try!(self.writer.write_all(bert_atom.as_slice()));
                try!(self.writer.write_all(time_atom.as_slice()));
            },
            REGEX_STRUCT_NAME => {
                let header = vec![BertTag::SmallTuple as u8, len as u8];
                let bert_atom = get_bert_atom();
                let regex_atom = get_atom("regex");
                try!(self.writer.write_all(header.as_slice()));
                try!(self.writer.write_all(bert_atom.as_slice()));
                try!(self.writer.write_all(regex_atom.as_slice()));
            },
            _ => {
                let mut header = vec![BertTag::LargeTuple as u8];
                let tuple_length = len as i32 + 1;
                header.write_i32::<BigEndian>(tuple_length).unwrap();
                try!(self.writer.write_all(header.as_slice()));

                let structure_name_atom = get_atom(_name);
                try!(self.writer.write_all(structure_name_atom.as_slice()));
            }
        }
        Ok(State::First)
    }

    #[inline]
    fn serialize_struct_elt<V: ser::Serialize>(
        &mut self, state: &mut State, key: &'static str, value: V
    ) -> Result<()> {
        *state = State::Rest;

        let header = vec![BertTag::SmallTuple as u8, 2u8];
        try!(self.writer.write_all(header.as_slice()));

        let field_atom = get_atom(key);
        try!(self.writer.write_all(field_atom.as_slice()));

        value.serialize(self)
    }

    #[inline]
    fn serialize_struct_end(&mut self, _state: State) -> Result<()> {
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

        let enum_name = get_atom(_name);
        try!(self.writer.write_all(enum_name.as_slice()));

        let mut variant_header = vec![BertTag::LargeTuple as u8];
        let variant_length = len as i32 + 1; // include variant name also
        variant_header.write_i32::<BigEndian>(variant_length).unwrap();
        try!(self.writer.write_all(variant_header.as_slice()));

        let variant_name = get_atom(variant);
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

        let field_atom = get_atom(key);
        try!(self.writer.write_all(field_atom.as_slice()));

        value.serialize(self)
    }

    #[inline]
    fn serialize_struct_variant_end(&mut self, _state: State) -> Result<()> {
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
    let mut binary = vec![ETF_VERSION];
    let data = try!(to_vec(value));
    binary.extend(data.iter().clone());
    Ok(binary)
}
