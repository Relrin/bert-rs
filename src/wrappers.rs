// Wrappers for types which are not supported by serde-rs and
// described by BERT specification.
use std::fmt;

use byteorder::{BigEndian, WriteBytesExt};
use num::bigint::{BigInt, Sign};
use serde::{ser, de};

use crate::types::{BertTag, BertBigInteger, BertTime, BertRegex, RegexOption};


pub const BIGNUM_STRUCT_NAME: &str = "_BertBigNumber";
pub const TIME_STRUCT_NAME: &str = "_BertTimeStruct";
pub const REGEX_STRUCT_NAME: &str = "_BertRegexStruct";
pub const REGEX_OPTION_ENUM_NAME: &str = "_BertRegexOptionsEnum";

impl ser::Serialize for BertBigInteger {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        let (num_sign, bytes) = self.to_bytes_le();
        let length = bytes.len();
        let sign: u8 = match num_sign {
            Sign::Plus | Sign::NoSign => 0,
            Sign::Minus => 1,
        };
        let mut binary: Vec<u8> = match length {
            0..=255 => {
                vec![BertTag::SmallBigNum as u8, length as u8, sign]
            },
            _ => {
                let mut binary = vec![BertTag::LargeBigNum as u8];
                binary.write_u32::<BigEndian>(length as u32).unwrap();
                binary.write_u8(sign).unwrap();
                binary
            }
        };
        binary.extend(bytes.iter());

        serializer.serialize_newtype_struct(BIGNUM_STRUCT_NAME, serde_bytes::Bytes::new(&binary))
    }
}


impl ser::Serialize for BertTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct(TIME_STRUCT_NAME, 5)?;
        state.serialize_field("megaseconds", &self.megaseconds)?;
        state.serialize_field("seconds", &self.seconds)?;
        state.serialize_field("microseconds", &self.microseconds)?;
        state.end()
    }
}


/// Wrapper to serialize regex options through the special REGEX_OPTION_ENUM_NAME path
struct RegexOptionsWrapper<'a>(&'a Vec<RegexOption>);

impl<'a> ser::Serialize for RegexOptionsWrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_newtype_struct(REGEX_OPTION_ENUM_NAME, self.0)
    }
}

impl ser::Serialize for BertRegex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct(REGEX_STRUCT_NAME, 4)?;
        state.serialize_field("source", &self.source)?;
        state.serialize_field("options", &RegexOptionsWrapper(&self.options))?;
        state.end()
    }
}

impl<'de> de::Deserialize<'de> for BertBigInteger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        // The deserializer presents bignum data as bytes: [sign, magnitude_bytes...]
        // via visit_byte_buf from parse_bignum_body
        struct BigIntVisitor;

        impl<'de> de::Visitor<'de> for BigIntVisitor {
            type Value = BertBigInteger;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("bignum bytes (sign byte followed by little-endian magnitude)")
            }

            fn visit_byte_buf<E: de::Error>(self, v: Vec<u8>) -> Result<BertBigInteger, E> {
                if v.is_empty() {
                    return Err(E::custom("empty bignum data"));
                }
                let sign_byte = v[0];
                let magnitude = &v[1..];
                let sign = if sign_byte == 0 { Sign::Plus } else { Sign::Minus };
                let bigint = BigInt::from_bytes_le(sign, magnitude);
                Ok(BertBigInteger(bigint))
            }
        }

        deserializer.deserialize_byte_buf(BigIntVisitor)
    }
}


impl<'de> de::Deserialize<'de> for BertTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        // The deserializer presents time fields as a seq of 3 i32 values
        // (after consuming the {bert, time} prefix)
        struct TimeVisitor;

        impl<'de> de::Visitor<'de> for TimeVisitor {
            type Value = BertTime;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a BERT time struct with megaseconds, seconds, microseconds")
            }

            fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<BertTime, A::Error> {
                let mega: i32 = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let sec: i32 = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let micro: i32 = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                Ok(BertTime::new(mega, sec, micro))
            }
        }

        deserializer.deserialize_struct(
            TIME_STRUCT_NAME,
            &["megaseconds", "seconds", "microseconds"],
            TimeVisitor,
        )
    }
}


impl<'de> de::Deserialize<'de> for BertRegex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        // The deserializer presents regex fields as a seq of [source_string, options_list]
        // (after consuming the {bert, regex} prefix)
        struct RegexVisitor;

        impl<'de> de::Visitor<'de> for RegexVisitor {
            type Value = BertRegex;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a BERT regex struct with source and options")
            }

            fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<BertRegex, A::Error> {
                let source: String = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let options: Vec<RegexOption> = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(BertRegex::new(&source, options))
            }
        }

        deserializer.deserialize_struct(
            REGEX_STRUCT_NAME,
            &["source", "options"],
            RegexVisitor,
        )
    }
}
