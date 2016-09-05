//! BERT Deserialization
//!
//! This module provides for BERT deserialization with the type `Deserializer`.
use std::io::{self, Read};

use byteorder::{BigEndian, ReadBytesExt};
use serde::de::{self, EnumVisitor, Visitor, Deserialize};

use super::errors::{Error, Result};
use super::types::{ETF_VERSION};


pub struct Deserializer<R: Read> {
    reader: R,
    header: Option<u8>,
}


impl<R: Read> Read for Deserializer<R> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}


impl<R: Read> Deserializer<R> {
    /// Creates the BERT parser from an `std::io::Read`.
    #[inline]
    pub fn new(reader: R) -> Deserializer<R> {
        Deserializer {
            reader: reader,
            header: None,
        }
    }

    /// The `Deserializer::end` method should be called after a value has
    /// been fully deserialized. This allows the `Deserializer` to validate
    /// that the input stream is at the end.
    #[inline]
    pub fn end(&mut self) -> Result<()> {
        if try!(self.read(&mut [0; 1])) == 0 {
            Ok(())
        } else {
            Err(Error::TrailingBytes)
        }
    }

    #[inline]
    fn parse_value<V: Visitor>(&mut self, visitor: V) -> Result<V::Value> {
        let header = self.header.unwrap();
        self.header = None;
        match header {
            97 => self.parse_unsigned_integer(header, visitor),
            98 => self.parse_integer(header, visitor),
            _ => Err(Error::InvalidTag)
        }
    }

    #[inline]
    fn parse_unsigned_integer<V: Visitor>(
        &mut self, header: u8, mut visitor: V
    ) -> Result<V::Value> {
        match header {
            97 => visitor.visit_u8(try!(self.read_u8())),
            _ => Err(Error::InvalidTag)
        }
    }

    #[inline]
    fn parse_integer<V: Visitor>(
        &mut self, header: u8, mut visitor: V
    ) -> Result<V::Value> {
        match header {
            98 => visitor.visit_i32(try!(self.read_i32::<BigEndian>())),
            _ => Err(Error::InvalidTag)
        }
    }
}


impl<R: Read> de::Deserializer for Deserializer<R> {
    type Error = Error;

    forward_deserialize!(
        deserialize_bool();
        deserialize_isize();
        deserialize_i8();
        deserialize_i16();
        deserialize_i32();
        deserialize_i64();
        deserialize_usize();
        deserialize_u8();
        deserialize_u16();
        deserialize_u32();
        deserialize_u64();
        deserialize_f32();
        deserialize_f64();
        deserialize_char();
        deserialize_str();
        deserialize_string();
        deserialize_bytes();
        deserialize_unit();
        deserialize_unit_struct(_name: &'static str,);
        deserialize_seq();
        deserialize_seq_fixed_size(_len: usize,);
        deserialize_map();
        deserialize_tuple_struct(_name: &'static str, _len: usize,);
        deserialize_tuple(_len: usize,);
        deserialize_struct(_name: &'static str, _fields: &'static [&'static str],);
        deserialize_struct_field();
        deserialize_ignored_any();
        deserialize_option();
        deserialize_newtype_struct(_name: &'static str,);
    );

    #[inline]
    fn deserialize<V: Visitor>(&mut self, visitor: V) -> Result<V::Value> {
        if self.header.is_none() {
            self.header = Some(try!(self.read_u8()));
        }

        let result = self.parse_value(visitor);
        self.header = None;
        result
    }

    #[inline]
    fn deserialize_enum<V: EnumVisitor>(
        &mut self, _enum: &'static str, _variants: &'static [&'static str],
        mut visitor: V
    ) -> Result<V::Value> {
        Err(Error::UnsupportedType)
    }
}


/// Decodes a BERT value from a `std::io::Read`.
#[inline]
pub fn from_reader<T: Deserialize, R: Read>(mut reader: R) -> Result<T> {
    let binary_header = try!(reader.read_u8());
    if binary_header != ETF_VERSION {
        let message = format!(
            "Data should start from the {} version number.",
            ETF_VERSION
        );
        Err(Error::Custom(message))
    } else {
        let mut de = Deserializer::new(reader);
        let value = try!(Deserialize::deserialize(&mut de));
        try!(de.end());
        Ok(value)
    }
}


/// Decodes a BERT value from a `&[u8]` slice.
#[inline]
pub fn from_slice<T: Deserialize>(v: &[u8]) -> Result<T> {
    from_reader(v)
}


/// Decode a BERT value from a binary stream (`&Vec<u8>`)
#[inline]
pub fn binary_to_term<T: Deserialize>(value: &Vec<u8>) -> Result<T>
    where T: de::Deserialize
{
    from_slice(value.as_slice())
}
