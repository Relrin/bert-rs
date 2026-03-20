//! BERT serialization
//!
//! This module provides for BERT serialization with the type `Serializer`.
use std::io;
use std::vec::Vec;

use byteorder::{BigEndian, WriteBytesExt};
use serde::ser;

use crate::errors::{Error, Result};
use crate::types::{ETF_VERSION, BertTag};
use crate::utils::{
    merge_terms, str_to_binary, get_atom, get_nil, get_bert_nil,
    get_bert_atom, get_empty_tuple, get_small_tuple
};
use crate::wrappers::{
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

struct BigNumSerializer<W> {
    writer: W,
}

impl<W: io::Write> BigNumSerializer<W> {
    pub fn new(writer: W) -> Self {
        BigNumSerializer { writer }
    }
}

impl<'a, W: io::Write> ser::Serializer for &'a mut BigNumSerializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    fn serialize_bool(self, _v: bool) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_i8(self, _v: i8) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_i16(self, _v: i16) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_i32(self, _v: i32) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_i64(self, _v: i64) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_u8(self, _v: u8) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_u16(self, _v: u16) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_u32(self, _v: u32) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_u64(self, _v: u64) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_f32(self, _v: f32) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_f64(self, _v: f64) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_char(self, _v: char) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_str(self, _v: &str) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_unit(self) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_none(self) -> Result<()> { Err(Error::UnsupportedType) }

    fn serialize_bytes(self, data: &[u8]) -> Result<()> {
        for byte in data {
            self.writer.write_all(&[*byte])?;
        }
        Ok(())
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(self, _value: &T) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_unit_variant(self, _name: &'static str, _idx: u32, _variant: &'static str) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(self, _name: &'static str, _value: &T) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(self, _name: &'static str, _idx: u32, _variant: &'static str, _value: &T) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::UnsupportedType)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::UnsupportedType)
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct> {
        Err(Error::UnsupportedType)
    }

    fn serialize_tuple_variant(self, _name: &'static str, _idx: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant> {
        Err(Error::UnsupportedType)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::UnsupportedType)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::UnsupportedType)
    }

    fn serialize_struct_variant(self, _name: &'static str, _idx: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant> {
        Err(Error::UnsupportedType)
    }
}

struct RegexOptionSerializer<W> {
    writer: W,
}

impl<W: io::Write> RegexOptionSerializer<W> {
    pub fn new(writer: W) -> Self {
        RegexOptionSerializer { writer }
    }
}

struct RegexOptionSeqSerializer<'a, W: 'a> {
    ser: &'a mut RegexOptionSerializer<W>,
    state: State,
}

impl<'a, W: io::Write> ser::SerializeSeq for RegexOptionSeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        self.state = State::Rest;
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<()> {
        match self.state {
            State::Empty => Ok(()),
            _ => {
                let nil = get_nil();
                self.ser.writer.write_all(nil.as_slice())?;
                Ok(())
            }
        }
    }
}

impl<'a, W: io::Write> ser::Serializer for &'a mut RegexOptionSerializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = RegexOptionSeqSerializer<'a, W>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    fn serialize_bool(self, _v: bool) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_i8(self, _v: i8) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_i16(self, _v: i16) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_i32(self, _v: i32) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_i64(self, _v: i64) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_u8(self, _v: u8) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_u16(self, _v: u16) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_u32(self, _v: u32) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_u64(self, _v: u64) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_f32(self, _v: f32) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_f64(self, _v: f64) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_char(self, _v: char) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_bytes(self, _v: &[u8]) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_unit(self) -> Result<()> { Err(Error::UnsupportedType) }
    fn serialize_none(self) -> Result<()> { Err(Error::UnsupportedType) }

    fn serialize_str(self, value: &str) -> Result<()> {
        let value = get_atom(value);
        self.writer.write_all(value.as_slice()).map_err(From::from)
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(self, _value: &T) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_unit_variant(self, _name: &'static str, _idx: u32, _variant: &'static str) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(self, _name: &'static str, _value: &T) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(self, _name: &'static str, _idx: u32, _variant: &'static str, _value: &T) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        match len {
            Some(0) => {
                let bert_nil_tuple = get_bert_nil();
                self.writer.write_all(bert_nil_tuple.as_slice())?;
                Ok(RegexOptionSeqSerializer { ser: self, state: State::Empty })
            }
            Some(list_length) => {
                let mut header = vec![BertTag::List as u8];
                header.write_i32::<BigEndian>(list_length as i32).unwrap();
                self.writer.write_all(header.as_slice())?;
                Ok(RegexOptionSeqSerializer { ser: self, state: State::First })
            }
            None => Ok(RegexOptionSeqSerializer { ser: self, state: State::Empty })
        }
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::UnsupportedType)
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct> {
        Err(Error::UnsupportedType)
    }

    fn serialize_tuple_variant(self, _name: &'static str, _idx: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant> {
        Err(Error::UnsupportedType)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::UnsupportedType)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::UnsupportedType)
    }

    fn serialize_struct_variant(self, _name: &'static str, _idx: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant> {
        Err(Error::UnsupportedType)
    }
}

pub struct Serializer<W> {
    writer: W,
}

impl<W: io::Write> Serializer<W> {
    pub fn new(writer: W) -> Serializer<W> {
        Serializer { writer }
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


// Compound serializer helpers
pub struct SeqSerializer<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
    state: State,
}

pub struct TupleSerializer<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
}

pub struct TupleStructSerializer<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
}

pub struct TupleVariantSerializer<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
}

pub struct MapSerializer<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
    state: State,
}

pub enum StructSerializer<'a, W: 'a> {
    /// Regular struct: fields are wrapped in SmallTuple(2, [Atom(field_name), value])
    Regular { ser: &'a mut Serializer<W> },
    /// BERT special types (time, regex): fields are raw values
    Bert { ser: &'a mut Serializer<W> },
}

pub struct StructVariantSerializer<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
}


impl<'a, W: io::Write> ser::SerializeSeq for SeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        self.state = State::Rest;
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<()> {
        match self.state {
            State::Empty => Ok(()),
            _ => {
                let nil = get_nil();
                self.ser.writer.write_all(nil.as_slice())?;
                Ok(())
            }
        }
    }
}


impl<'a, W: io::Write> ser::SerializeTuple for TupleSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}


impl<'a, W: io::Write> ser::SerializeTupleStruct for TupleStructSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}


impl<'a, W: io::Write> ser::SerializeTupleVariant for TupleVariantSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}


impl<'a, W: io::Write> ser::SerializeMap for MapSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + ser::Serialize>(&mut self, key: &T) -> Result<()> {
        self.state = State::Rest;
        let tuple_header = vec![BertTag::SmallTuple as u8, 2u8];
        self.ser.writer.write_all(tuple_header.as_slice())?;
        key.serialize(&mut *self.ser)
    }

    fn serialize_value<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<()> {
        if self.state == State::Rest {
            let nil_atom = get_nil();
            self.ser.writer.write_all(nil_atom.as_slice())?;
        }
        Ok(())
    }
}


impl<'a, W: io::Write> ser::SerializeStruct for StructSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, key: &'static str, value: &T) -> Result<()> {
        match self {
            StructSerializer::Regular { ser } => {
                let header = vec![BertTag::SmallTuple as u8, 2u8];
                ser.writer.write_all(header.as_slice())?;

                let field_atom = get_atom(key);
                ser.writer.write_all(field_atom.as_slice())?;

                value.serialize(&mut **ser)
            }
            StructSerializer::Bert { ser } => {
                value.serialize(&mut **ser)
            }
        }
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}


impl<'a, W: io::Write> ser::SerializeStructVariant for StructVariantSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, key: &'static str, value: &T) -> Result<()> {
        let header = vec![BertTag::SmallTuple as u8, 2u8];
        self.ser.writer.write_all(header.as_slice())?;

        let field_atom = get_atom(key);
        self.ser.writer.write_all(field_atom.as_slice())?;

        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W: io::Write> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a, W>;
    type SerializeTuple = TupleSerializer<'a, W>;
    type SerializeTupleStruct = TupleStructSerializer<'a, W>;
    type SerializeTupleVariant = TupleVariantSerializer<'a, W>;
    type SerializeMap = MapSerializer<'a, W>;
    type SerializeStruct = StructSerializer<'a, W>;
    type SerializeStructVariant = StructVariantSerializer<'a, W>;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<()> {
        let boolean_string = value.to_string();

        let bert_atom = get_bert_atom();
        let boolean_atom = get_atom(&boolean_string);

        let binary = merge_terms(bert_atom, boolean_atom);
        let tuple = get_small_tuple(2, binary);
        self.writer.write_all(tuple.as_slice()).map_err(From::from)
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<()> {
        self.serialize_i32(value as i32)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<()> {
        self.serialize_i32(value as i32)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<()> {
        let mut binary = vec![];
        binary.write_i32::<BigEndian>(value).unwrap();
        self.generate_term(BertTag::Integer, binary)?;
        Ok(())
    }

    #[inline]
    fn serialize_i64(self, _value: i64) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> Result<()> {
        self.generate_term(BertTag::SmallInteger, vec![value])
    }

    #[inline]
    fn serialize_u16(self, _value: u16) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_u32(self, _value: u32) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_u64(self, _value: u64) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<()> {
        self.serialize_f64(value as f64)
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<()> {
        let mut binary = vec![];
        binary.write_f64::<BigEndian>(value).unwrap();
        self.generate_term(BertTag::NewFloat, binary)
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<()> {
        self.serialize_str(&value.to_string())
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        let binary_string = str_to_binary(value);
        self.generate_term(BertTag::String, binary_string)
    }

    #[inline]
    fn serialize_bytes(self, data: &[u8]) -> Result<()> {
        let length = data.len();

        let mut header = vec![BertTag::Binary as u8];
        header.write_i32::<BigEndian>(length as i32).unwrap();
        self.writer.write_all(header.as_slice())?;

        for byte in data {
            self.writer.write_all(&[*byte])?;
        }

        Ok(())
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        let nil = get_nil();
        self.writer.write_all(nil.as_slice()).map_err(From::from)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self, _name: &'static str, _variant_index: u32,
        variant: &'static str
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self, name: &'static str, value: &T
    ) -> Result<()> {
        match name {
            BIGNUM_STRUCT_NAME => {
                let mut bignum_serializer = BigNumSerializer::new(&mut self.writer);
                value.serialize(&mut bignum_serializer)
            },
            REGEX_OPTION_ENUM_NAME => {
                let mut regex_options_serializer = RegexOptionSerializer::new(&mut self.writer);
                value.serialize(&mut regex_options_serializer)
            },
            _ => {
                let header = vec![BertTag::SmallTuple as u8, 2u8];
                self.writer.write_all(header.as_slice())?;

                let structure_name_atom = get_atom(name);
                self.writer.write_all(structure_name_atom.as_slice())?;

                value.serialize(self)
            }
        }
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self, name: &'static str, _variant_index: u32,
        variant: &'static str, value: &T
    ) -> Result<()> {
        let header = vec![BertTag::SmallTuple as u8, 2u8];
        self.writer.write_all(header.as_slice())?;

        let enum_atom = get_atom(name);
        self.writer.write_all(enum_atom.as_slice())?;

        let variant_header = vec![BertTag::SmallTuple as u8, 2u8];
        self.writer.write_all(variant_header.as_slice())?;

        let variant_atom = get_atom(variant);
        self.writer.write_all(variant_atom.as_slice())?;

        value.serialize(self)
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T: ?Sized + ser::Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        match len {
            Some(0) => {
                let bert_nil_tuple = get_bert_nil();
                self.writer.write_all(bert_nil_tuple.as_slice())?;
                Ok(SeqSerializer { ser: self, state: State::Empty })
            }
            Some(list_length) => {
                let mut header = vec![BertTag::List as u8];
                header.write_i32::<BigEndian>(list_length as i32).unwrap();
                self.writer.write_all(header.as_slice())?;
                Ok(SeqSerializer { ser: self, state: State::First })
            }
            None => Ok(SeqSerializer { ser: self, state: State::Empty })
        }
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        match len {
            0 => {
                let empty_tuple = get_empty_tuple();
                self.writer.write_all(empty_tuple.as_slice())?;
            }
            1..=255 => {
                let header = vec![BertTag::SmallTuple as u8, len as u8];
                self.writer.write_all(header.as_slice())?;
            }
            _ => {
                let mut header = vec![BertTag::LargeTuple as u8];
                header.write_i32::<BigEndian>(len as i32).unwrap();
                self.writer.write_all(header.as_slice())?;
            }
        }
        Ok(TupleSerializer { ser: self })
    }

    #[inline]
    fn serialize_tuple_struct(
        self, name: &'static str, len: usize
    ) -> Result<Self::SerializeTupleStruct> {
        let tuple_size = len + 1; // include name of entity
        let mut header = vec![BertTag::LargeTuple as u8];
        header.write_i32::<BigEndian>(tuple_size as i32).unwrap();
        self.writer.write_all(header.as_slice())?;

        let structure_name = get_atom(name);
        self.writer.write_all(structure_name.as_slice())?;

        Ok(TupleStructSerializer { ser: self })
    }

    #[inline]
    fn serialize_tuple_variant(
        self, name: &'static str, _variant_index: u32,
        variant: &'static str, len: usize
    ) -> Result<Self::SerializeTupleVariant> {
        let mut header = vec![BertTag::LargeTuple as u8];
        header.write_i32::<BigEndian>(2i32).unwrap();
        self.writer.write_all(header.as_slice())?;

        let enum_name = get_atom(name);
        self.writer.write_all(enum_name.as_slice())?;

        let mut variant_header = vec![BertTag::LargeTuple as u8];
        let variant_length = len as i32 + 1;
        variant_header.write_i32::<BigEndian>(variant_length).unwrap();
        self.writer.write_all(variant_header.as_slice())?;

        let variant_name = get_atom(variant);
        self.writer.write_all(variant_name.as_slice())?;

        Ok(TupleVariantSerializer { ser: self })
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        let header = vec![BertTag::SmallTuple as u8, 3u8];
        let bert_atom = get_bert_atom();
        let dict_atom = get_atom("dict");

        self.writer.write_all(header.as_slice())?;
        self.writer.write_all(bert_atom.as_slice())?;
        self.writer.write_all(dict_atom.as_slice())?;

        let mut list_header: Vec<u8> = vec![];

        let state = match len {
            Some(0) | None => {
                list_header.push(BertTag::Nil as u8);
                State::Empty
            }
            Some(length) => {
                list_header.push(BertTag::List as u8);
                list_header.write_i32::<BigEndian>(length as i32).unwrap();
                State::First
            }
        };

        self.writer.write_all(list_header.as_slice())?;
        Ok(MapSerializer { ser: self, state })
    }

    #[inline]
    fn serialize_struct(
        self, name: &'static str, len: usize
    ) -> Result<Self::SerializeStruct> {
        match name {
            TIME_STRUCT_NAME => {
                let header = vec![BertTag::SmallTuple as u8, len as u8];
                let bert_atom = get_bert_atom();
                let time_atom = get_atom("time");
                self.writer.write_all(header.as_slice())?;
                self.writer.write_all(bert_atom.as_slice())?;
                self.writer.write_all(time_atom.as_slice())?;
                Ok(StructSerializer::Bert { ser: self })
            }
            REGEX_STRUCT_NAME => {
                let header = vec![BertTag::SmallTuple as u8, len as u8];
                let bert_atom = get_bert_atom();
                let regex_atom = get_atom("regex");
                self.writer.write_all(header.as_slice())?;
                self.writer.write_all(bert_atom.as_slice())?;
                self.writer.write_all(regex_atom.as_slice())?;
                Ok(StructSerializer::Bert { ser: self })
            }
            _ => {
                let mut header = vec![BertTag::LargeTuple as u8];
                let tuple_length = len as i32 + 1;
                header.write_i32::<BigEndian>(tuple_length).unwrap();
                self.writer.write_all(header.as_slice())?;

                let structure_name_atom = get_atom(name);
                self.writer.write_all(structure_name_atom.as_slice())?;
                Ok(StructSerializer::Regular { ser: self })
            }
        }
    }

    #[inline]
    fn serialize_struct_variant(
        self, name: &'static str, _variant_index: u32,
        variant: &'static str, len: usize
    ) -> Result<Self::SerializeStructVariant> {
        let mut header = vec![BertTag::LargeTuple as u8];
        header.write_i32::<BigEndian>(2i32).unwrap();
        self.writer.write_all(header.as_slice())?;

        let enum_name = get_atom(name);
        self.writer.write_all(enum_name.as_slice())?;

        let mut variant_header = vec![BertTag::LargeTuple as u8];
        let variant_length = len as i32 + 1;
        variant_header.write_i32::<BigEndian>(variant_length).unwrap();
        self.writer.write_all(variant_header.as_slice())?;

        let variant_name = get_atom(variant);
        self.writer.write_all(variant_name.as_slice())?;

        Ok(StructVariantSerializer { ser: self })
    }
}


/// Encode the passed value into a `[u8]` writer
#[inline]
pub fn to_writer<W, T>(writer: &mut W, value: &T) -> Result<()>
where
    W: io::Write,
    T: ser::Serialize,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)?;
    Ok(())
}


/// Encode the specified struct into a `[u8]` buffer
#[inline]
pub fn to_vec<T: ser::Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}


/// Convert passed value to a BERT representation
#[inline]
pub fn term_to_binary<T: ser::Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut binary = vec![ETF_VERSION];
    let data = to_vec(value)?;
    binary.extend(data.iter());
    Ok(binary)
}
