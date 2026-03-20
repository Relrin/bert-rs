//! BERT Deserialization
//!
//! This module provides for BERT deserialization with the type `Deserializer`
use std::io::{self, Read};
use std::str::FromStr;

use byteorder::{BigEndian, ReadBytesExt};
use serde::de::{self, Visitor, DeserializeSeed};

use crate::errors::{Error, Result};
use crate::types::ETF_VERSION;


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
    /// Creates the BERT parser from an `std::io::Read`
    #[inline]
    pub fn new(reader: R) -> Deserializer<R> {
        Deserializer {
            reader,
            header: None,
        }
    }

    /// The `Deserializer::end` method should be called after a value has
    /// been fully deserialized. This allows the `Deserializer` to validate
    /// that the input stream is at the end
    #[inline]
    pub fn end(&mut self) -> Result<()> {
        if self.read(&mut [0; 1])? == 0 {
            Ok(())
        } else {
            Err(Error::TrailingBytes)
        }
    }

    /// Ensure a header byte is available (read one if not already peeked)
    #[inline]
    fn ensure_header(&mut self) -> Result<u8> {
        if let Some(h) = self.header {
            Ok(h)
        } else {
            let h = self.read_u8()?;
            self.header = Some(h);
            Ok(h)
        }
    }

    #[inline]
    fn read_string(&mut self, len: usize) -> io::Result<String> {
        let reader = self.reader.by_ref();
        let mut string_buffer = String::with_capacity(len);
        reader.take(len as u64).read_to_string(&mut string_buffer)?;
        string_buffer = string_buffer.replace("\u{0000}", "");
        Ok(string_buffer)
    }

    /// Read an atom value (after the tag byte has been consumed).
    /// Returns the atom string
    #[inline]
    fn read_atom_value(&mut self) -> Result<String> {
        let length = self.read_i16::<BigEndian>()?;
        let string = self.read_string(length as usize)?;
        Ok(string)
    }

    #[inline]
    fn parse_value<'de, V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value> {
        let header = self.header.unwrap();
        self.header = None;
        match header {
            70 | 99 => self.parse_float(header, visitor),
            97 => self.parse_unsigned_integer(visitor),
            98 => self.parse_integer(visitor),
            100 => self.parse_atom(visitor),
            104 => self.parse_small_tuple(visitor),
            105 => self.parse_large_tuple(visitor),
            106 => self.parse_nil(visitor),
            107 => self.parse_string(visitor),
            108 => self.parse_list(visitor),
            109 => self.parse_binary(visitor),
            110 => self.parse_small_bignum(visitor),
            111 => self.parse_large_bignum(visitor),
            _ => Err(Error::InvalidTag)
        }
    }

    #[inline]
    fn parse_float<'de, V: Visitor<'de>>(
        &mut self, header: u8, visitor: V
    ) -> Result<V::Value> {
        match header {
            70 => {
                let value = self.read_f64::<BigEndian>()?;
                visitor.visit_f64(value)
            },
            99 => {
                let float_str = self.read_string(31)?;
                let value = f32::from_str(&float_str)?;
                visitor.visit_f64(value as f64)
            },
            _ => Err(Error::InvalidTag)
        }
    }

    #[inline]
    fn parse_unsigned_integer<'de, V: Visitor<'de>>(
        &mut self, visitor: V
    ) -> Result<V::Value> {
        visitor.visit_u8(self.read_u8()?)
    }

    #[inline]
    fn parse_integer<'de, V: Visitor<'de>>(
        &mut self, visitor: V
    ) -> Result<V::Value> {
        visitor.visit_i32(self.read_i32::<BigEndian>()?)
    }

    #[inline]
    fn parse_atom<'de, V: Visitor<'de>>(
        &mut self, visitor: V
    ) -> Result<V::Value> {
        let string = self.read_atom_value()?;
        visitor.visit_string(string)
    }

    #[inline]
    fn parse_string<'de, V: Visitor<'de>>(
        &mut self, visitor: V
    ) -> Result<V::Value> {
        let length = self.read_i16::<BigEndian>()?;
        let string = self.read_string(length as usize)?;
        visitor.visit_string(string)
    }

    #[inline]
    fn parse_binary<'de, V: Visitor<'de>>(
        &mut self, visitor: V
    ) -> Result<V::Value> {
        let length = self.read_i32::<BigEndian>()? as usize;
        visitor.visit_seq(BinarySeqAccess::new(self, length))
    }

    #[inline]
    fn parse_nil<'de, V: Visitor<'de>>(
        &mut self, visitor: V
    ) -> Result<V::Value> {
        visitor.visit_unit()
    }

    #[inline]
    fn parse_list<'de, V: Visitor<'de>>(
        &mut self, visitor: V
    ) -> Result<V::Value> {
        let length = self.read_i32::<BigEndian>()? as usize;
        let result = visitor.visit_seq(ListSeqAccess::new(self, length))?;
        // Consume the trailing Nil byte after the list elements
        let tail = self.read_u8()?;
        if tail != 106 {
            return Err(Error::InvalidTag);
        }
        Ok(result)
    }

    #[inline]
    fn parse_small_tuple<'de, V: Visitor<'de>>(
        &mut self, visitor: V
    ) -> Result<V::Value> {
        let arity = self.read_u8()? as usize;
        visitor.visit_seq(TupleSeqAccess::new(self, arity))
    }

    #[inline]
    fn parse_large_tuple<'de, V: Visitor<'de>>(
        &mut self, visitor: V
    ) -> Result<V::Value> {
        let arity = self.read_i32::<BigEndian>()? as usize;
        visitor.visit_seq(TupleSeqAccess::new(self, arity))
    }

    #[inline]
    fn parse_small_bignum<'de, V: Visitor<'de>>(
        &mut self, visitor: V
    ) -> Result<V::Value> {
        let n = self.read_u8()? as usize;
        self.parse_bignum_body(n, visitor)
    }

    #[inline]
    fn parse_large_bignum<'de, V: Visitor<'de>>(
        &mut self, visitor: V
    ) -> Result<V::Value> {
        let n = self.read_i32::<BigEndian>()? as usize;
        self.parse_bignum_body(n, visitor)
    }

    #[inline]
    fn parse_bignum_body<'de, V: Visitor<'de>>(
        &mut self, n: usize, visitor: V
    ) -> Result<V::Value> {
        let sign = self.read_u8()?;
        let mut magnitude = vec![0u8; n];
        self.reader.read_exact(&mut magnitude)?;
        // Encode as: [sign, magnitude_bytes...] so the custom deserializer can reconstruct
        let mut data = Vec::with_capacity(1 + n);
        data.push(sign);
        data.extend_from_slice(&magnitude);
        visitor.visit_byte_buf(data)
    }

    /// Parse a BERT boolean tuple `{bert, true}` or `{bert, false}`.
    /// Assumes the SmallTuple tag + arity(2) have already been consumed
    fn parse_bert_bool<'de, V: Visitor<'de>>(
        &mut self, visitor: V
    ) -> Result<V::Value> {
        // Read the second element which should be the atom "true" or "false"
        let tag = self.read_u8()?;
        if tag != 100 {
            return Err(Error::InvalidTag);
        }
        let atom = self.read_atom_value()?;
        match atom.as_str() {
            "true" => visitor.visit_bool(true),
            "false" => visitor.visit_bool(false),
            _ => Err(Error::Custom(format!("expected true or false atom, got {}", atom))),
        }
    }

    /// Parse a BERT dict tuple `{bert, dict, list}`.
    /// Assumes SmallTuple(3) + atom("bert") + atom("dict") have been consumed
    fn parse_bert_dict<'de, V: Visitor<'de>>(
        &mut self, visitor: V
    ) -> Result<V::Value> {
        let tag = self.read_u8()?;
        match tag {
            106 => {
                // Nil - empty dict
                visitor.visit_map(DictMapAccess::new(self, 0))
            }
            108 => {
                let length = self.read_i32::<BigEndian>()? as usize;
                let result = visitor.visit_map(DictMapAccess::new(self, length))?;
                // Consume trailing nil
                let tail = self.read_u8()?;
                if tail != 106 {
                    return Err(Error::InvalidTag);
                }
                Ok(result)
            }
            _ => Err(Error::InvalidTag),
        }
    }
}


impl<'de, 'a, R: Read> de::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.ensure_header()?;
        let header = self.header.unwrap();

        // Special handling for tuples starting with atom "bert"
        if header == 104 {
            // Peek: read arity
            self.header = None;
            let arity = self.read_u8()? as usize;

            if arity >= 2 {
                // Read first element tag
                let first_tag = self.read_u8()?;
                if first_tag == 100 {
                    let atom = self.read_atom_value()?;
                    if atom == "bert" {
                        // This is a special BERT type
                        if arity == 2 {
                            // Could be {bert, true}, {bert, false}, {bert, nil}
                            return self.parse_bert_bool(visitor);
                        } else if arity == 3 {
                            // Could be {bert, dict, ...}
                            let tag2 = self.read_u8()?;
                            if tag2 == 100 {
                                let atom2 = self.read_atom_value()?;
                                if atom2 == "dict" {
                                    return self.parse_bert_dict(visitor);
                                }
                            }
                            return Err(Error::InvalidTag);
                        } else if arity == 5 {
                            // Could be {bert, time, mega, sec, micro}
                            // Consume the "time" atom, then read 3 i32 values as a seq
                            let tag2 = self.read_u8()?;
                            if tag2 == 100 {
                                let atom2 = self.read_atom_value()?;
                                if atom2 == "time" {
                                    return visitor.visit_seq(TupleSeqAccess::new(self, 3));
                                }
                            }
                            return Err(Error::InvalidTag);
                        } else if arity == 4 {
                            // Could be {bert, regex, source, options}
                            let tag2 = self.read_u8()?;
                            if tag2 == 100 {
                                let atom2 = self.read_atom_value()?;
                                if atom2 == "regex" {
                                    return visitor.visit_seq(TupleSeqAccess::new(self, 2));
                                }
                            }
                            return Err(Error::InvalidTag);
                        }
                    }
                    // Not "bert" atom - it's a generic tuple where first elem was an atom
                    // We've consumed the atom, remaining arity-1 elements
                    // Present as seq with the atom string as first element
                    return visitor.visit_seq(PrependedSeqAccess::new(
                        self,
                        PrependedValue::Str(atom),
                        arity - 1,
                    ));
                }
                // First element is not an atom. Buffer its tag and present as generic tuple
                self.header = Some(first_tag);
                return visitor.visit_seq(TupleSeqAccess::new(self, arity));
            }
            // arity 0 or 1
            return visitor.visit_seq(TupleSeqAccess::new(self, arity));
        }

        self.parse_value(visitor)
    }

    #[inline]
    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.ensure_header()?;
        let header = self.header.unwrap();
        self.header = None;

        if header == 104 {
            let arity = self.read_u8()?;
            if arity != 2 {
                return Err(Error::InvalidTag);
            }
            // Read "bert" atom
            let tag = self.read_u8()?;
            if tag != 100 {
                return Err(Error::InvalidTag);
            }
            let atom = self.read_atom_value()?;
            if atom != "bert" {
                return Err(Error::Custom(format!("expected bert atom, got {}", atom)));
            }
            return self.parse_bert_bool(visitor);
        }
        Err(Error::InvalidTag)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.ensure_header()?;
        let header = self.header.unwrap();

        if header == 106 {
            // Nil => None
            self.header = None;
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.ensure_header()?;
        self.parse_value(visitor)
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value> {
        self.ensure_header()?;
        self.parse_value(visitor)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self, _name: &'static str, _len: usize, visitor: V
    ) -> Result<V::Value> {
        self.ensure_header()?;
        self.parse_value(visitor)
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.ensure_header()?;
        let header = self.header.unwrap();
        self.header = None;

        if header == 104 {
            let arity = self.read_u8()?;
            if arity != 3 {
                return Err(Error::InvalidTag);
            }
            // Read "bert" atom
            let tag = self.read_u8()?;
            if tag != 100 { return Err(Error::InvalidTag); }
            let atom = self.read_atom_value()?;
            if atom != "bert" {
                return Err(Error::Custom(format!("expected bert atom, got {}", atom)));
            }
            // Read "dict" atom
            let tag = self.read_u8()?;
            if tag != 100 { return Err(Error::InvalidTag); }
            let atom = self.read_atom_value()?;
            if atom != "dict" {
                return Err(Error::Custom(format!("expected dict atom, got {}", atom)));
            }
            return self.parse_bert_dict(visitor);
        }
        Err(Error::InvalidTag)
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self, name: &'static str, _fields: &'static [&'static str], visitor: V
    ) -> Result<V::Value> {
        use crate::wrappers::{TIME_STRUCT_NAME, REGEX_STRUCT_NAME, BIGNUM_STRUCT_NAME};

        self.ensure_header()?;
        let header = self.header.unwrap();

        match name {
            TIME_STRUCT_NAME | REGEX_STRUCT_NAME => {
                // These are encoded as {bert, type_atom, ...fields}
                // The wrapper Deserialize impls call deserialize_struct,
                // and we need to present the fields as a seq
                self.header = None;
                if header == 104 {
                    let arity = self.read_u8()? as usize;
                    // Skip "bert" atom
                    let tag = self.read_u8()?;
                    if tag != 100 { return Err(Error::InvalidTag); }
                    let _bert = self.read_atom_value()?;
                    // Skip type atom (time/regex)
                    let tag = self.read_u8()?;
                    if tag != 100 { return Err(Error::InvalidTag); }
                    let _type_atom = self.read_atom_value()?;
                    // Remaining elements are the fields
                    let remaining = arity - 2;
                    return visitor.visit_seq(TupleSeqAccess::new(self, remaining));
                }
                Err(Error::InvalidTag)
            }
            BIGNUM_STRUCT_NAME => {
                // Bignum is encoded as newtype_struct with bytes
                self.deserialize_newtype_struct(name, visitor)
            }
            _ => {
                // Generic struct: LargeTuple(N+1, [Atom(name), field_pairs...])
                self.header = None;
                match header {
                    104 => {
                        let arity = self.read_u8()? as usize;
                        // Read and skip the struct name atom
                        let tag = self.read_u8()?;
                        if tag != 100 { return Err(Error::InvalidTag); }
                        let _struct_name = self.read_atom_value()?;
                        // Each field is SmallTuple(2, [Atom(field_name), value])
                        // We present as a map
                        visitor.visit_map(StructMapAccess::new(self, arity - 1))
                    }
                    105 => {
                        let arity = self.read_i32::<BigEndian>()? as usize;
                        let tag = self.read_u8()?;
                        if tag != 100 { return Err(Error::InvalidTag); }
                        let _struct_name = self.read_atom_value()?;
                        visitor.visit_map(StructMapAccess::new(self, arity - 1))
                    }
                    _ => Err(Error::InvalidTag),
                }
            }
        }
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self, _name: &'static str, visitor: V
    ) -> Result<V::Value> {
        self.ensure_header()?;
        self.parse_value(visitor)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self, _name: &'static str, _variants: &'static [&'static str],
        _visitor: V
    ) -> Result<V::Value> {
        Err(Error::UnsupportedType)
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    serde::forward_to_deserialize_any! {
        u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string
        bytes byte_buf unit unit_struct
    }
}

struct BinarySeqAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    remaining: usize,
}

impl<'a, R: Read> BinarySeqAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>, length: usize) -> Self {
        BinarySeqAccess { de, remaining: length }
    }
}

impl<'de, 'a, R: Read> de::SeqAccess<'de> for BinarySeqAccess<'a, R> {
    type Error = Error;

    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        if self.remaining == 0 {
            return Ok(None);
        }
        self.remaining -= 1;
        let byte = self.de.read_u8()?;
        seed.deserialize(de::value::U8Deserializer::new(byte)).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

struct ListSeqAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    remaining: usize,
}

impl<'a, R: Read> ListSeqAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>, length: usize) -> Self {
        ListSeqAccess { de, remaining: length }
    }
}

impl<'de, 'a, R: Read> de::SeqAccess<'de> for ListSeqAccess<'a, R> {
    type Error = Error;

    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        if self.remaining == 0 {
            return Ok(None);
        }
        self.remaining -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}


struct TupleSeqAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    remaining: usize,
}

impl<'a, R: Read> TupleSeqAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>, arity: usize) -> Self {
        TupleSeqAccess { de, remaining: arity }
    }
}

impl<'de, 'a, R: Read> de::SeqAccess<'de> for TupleSeqAccess<'a, R> {
    type Error = Error;

    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        if self.remaining == 0 {
            return Ok(None);
        }
        self.remaining -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}


// For generic tuples with a consumed first atom
enum PrependedValue {
    Str(String),
}

struct PrependedSeqAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    prepended: Option<PrependedValue>,
    remaining: usize,
}

impl<'a, R: Read> PrependedSeqAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>, prepended: PrependedValue, remaining: usize) -> Self {
        PrependedSeqAccess { de, prepended: Some(prepended), remaining }
    }
}

impl<'de, 'a, R: Read> de::SeqAccess<'de> for PrependedSeqAccess<'a, R> {
    type Error = Error;

    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        if let Some(val) = self.prepended.take() {
            match val {
                PrependedValue::Str(s) => {
                    seed.deserialize(de::value::StringDeserializer::new(s)).map(Some)
                }
            }
        } else if self.remaining == 0 {
            Ok(None)
        } else {
            self.remaining -= 1;
            seed.deserialize(&mut *self.de).map(Some)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        let extra = if self.prepended.is_some() { 1 } else { 0 };
        Some(self.remaining + extra)
    }
}

struct DictMapAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    remaining: usize,
}

impl<'a, R: Read> DictMapAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>, length: usize) -> Self {
        DictMapAccess { de, remaining: length }
    }
}

impl<'de, 'a, R: Read> de::MapAccess<'de> for DictMapAccess<'a, R> {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        if self.remaining == 0 {
            return Ok(None);
        }
        self.remaining -= 1;
        // Each entry is SmallTuple(2, [key, value])
        let tag = self.de.read_u8()?;
        if tag != 104 {
            return Err(Error::InvalidTag);
        }
        let arity = self.de.read_u8()?;
        if arity != 2 {
            return Err(Error::InvalidTag);
        }
        // Deserialize key
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        seed.deserialize(&mut *self.de)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

struct StructMapAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    remaining: usize,
}

impl<'a, R: Read> StructMapAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>, length: usize) -> Self {
        StructMapAccess { de, remaining: length }
    }
}

impl<'de, 'a, R: Read> de::MapAccess<'de> for StructMapAccess<'a, R> {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        if self.remaining == 0 {
            return Ok(None);
        }
        self.remaining -= 1;
        // Each field is SmallTuple(2, [Atom(field_name), value])
        let tag = self.de.read_u8()?;
        if tag != 104 {
            return Err(Error::InvalidTag);
        }
        let arity = self.de.read_u8()?;
        if arity != 2 {
            return Err(Error::InvalidTag);
        }
        // Read the field name atom
        let atom_tag = self.de.read_u8()?;
        if atom_tag != 100 {
            return Err(Error::InvalidTag);
        }
        let field_name = self.de.read_atom_value()?;
        seed.deserialize(de::value::StringDeserializer::new(field_name)).map(Some)
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        seed.deserialize(&mut *self.de)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

/// Decodes a BERT value from a `std::io::Read`.
#[inline]
pub fn from_reader<T, R: Read>(mut reader: R) -> Result<T>
where
    T: de::DeserializeOwned,
{
    let binary_header = reader.read_u8()?;
    if binary_header != ETF_VERSION {
        let message = format!(
            "Data should start from the {} version number.",
            ETF_VERSION
        );
        Err(Error::Custom(message))
    } else {
        let mut de = Deserializer::new(reader);
        let value = T::deserialize(&mut de)?;
        de.end()?;
        Ok(value)
    }
}

/// Decodes a BERT value from a `&[u8]` slice.
#[inline]
pub fn from_slice<T: de::DeserializeOwned>(v: &[u8]) -> Result<T> {
    from_reader(v)
}

/// Decode a BERT value from a binary stream (`&Vec<u8>`)
#[inline]
pub fn binary_to_term<T: de::DeserializeOwned>(value: &Vec<u8>) -> Result<T> {
    from_slice(value.as_slice())
}
