//! BERT Deserialization
//!
//! This module provides for BERT deserialization with the type `Deserializer`.
use std::{f32};
use std::io::{self, Read};
use std::str::FromStr;

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
    fn read_string(&mut self, len: usize) -> io::Result<String> {
        let reader = self.reader.by_ref();
        let mut string_buffer = String::with_capacity(len);
        try!(reader.take(len as u64).read_to_string(&mut string_buffer));
        string_buffer = string_buffer.replace("\u{0000}", "");
        Ok(string_buffer)
    }

    #[inline]
    fn parse_value<V: Visitor>(&mut self, visitor: V) -> Result<V::Value> {
        let header = self.header.unwrap();
        self.header = None;
        match header {
            70 | 99 => self.parse_float(header, visitor),
            97 => self.parse_unsigned_integer(header, visitor),
            98 => self.parse_integer(header, visitor),
            _ => Err(Error::InvalidTag)
        }
    }

    #[inline]
    fn parse_float<V: Visitor>(
        &mut self, header: u8, mut visitor: V
    ) -> Result<V::Value> {
        match header {
            70 => {
                let value = try!(self.read_f64::<BigEndian>());
                visitor.visit_f64(value)
            },
            99 => {
                let float_str = try!(self.read_string(31));
                let value = try!(f32::from_str(&float_str));
                visitor.visit_f64(value as f64)
            },
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

    forward_to_deserialize! {
        bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 char
        str string unit seq seq_fixed_size bytes option map unit_struct
        tuple_struct struct struct_field tuple ignored_any newtype_struct
    }

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
