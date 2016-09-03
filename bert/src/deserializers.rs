//! BERT Deserialization
//!
//! This module provides for BERT deserialization with the type `Deserializer`.
use std::io::{self, Read};

use byteorder::{BigEndian, ReadBytesExt};
use serde::bytes::ByteBuf;
use serde::de::{self, EnumVisitor, Visitor, Deserialize};

use super::errors::{Error, Result};


macro_rules! forward_deserialize {
    ($($name:ident($($arg:ident: $ty:ty,)*);)*) => {
        $(#[inline]
        fn $name<V: Visitor>(&mut self, $($arg: $ty,)* visitor: V) -> Result<V::Value> {
            self.deserialize(visitor)
        })*
    }
}


pub struct Deserializer<R: Read> {
    reader: R,
    first: Option<u8>,
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
            first: None,
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
        let first = self.first.unwrap();
        self.first = None;
        match first {
            _ => unreachable!(),
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
    );

    #[inline]
    fn deserialize<V: Visitor>(
        &mut self, visitor: V
    ) -> Result<V::Value> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn deserialize_option<V: Visitor>(
        &mut self, mut visitor: V
    ) -> Result<V::Value> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn deserialize_newtype_struct<V>(
        &mut self, _name: &'static str, mut visitor: V
    ) -> Result<V::Value> where V: de::Visitor {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn deserialize_enum<V: EnumVisitor>(
        &mut self, _enum: &'static str, _variants: &'static [&'static str],
        mut visitor: V
    ) -> Result<V::Value> {
        Err(Error::UnsupportedType)
    }
}


fn from_trait<R, T>(read: R) -> Result<T>
    where R: Read, T: de::Deserialize,
{
    let mut de = Deserializer::new(read);
    let value = try!(de::Deserialize::deserialize(&mut de));

    // Make sure the whole stream has been consumed.
    try!(de.end());
    Ok(value)
}


/// Decode a BERT value from binary stream (`Vec<u8>`)
pub fn binary_to_term<T>(value: Vec<u8>) -> Result<T>
    where T: de::Deserialize
{
    from_trait(value.as_slice())
}
