// Wrappers for types which are not supported by serde-rs and
// described by BERT specification.
use std::result::Result;

use byteorder::{BigEndian, WriteBytesExt};
use num::bigint::{Sign};
use serde::bytes;
use serde::ser;

use types::{BertTag, BertBigInteger};


pub const BIGNUM_STRUCT_NAME: &'static str = "_BertBigNumber";


impl ser::Serialize for BertBigInteger {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: ser::Serializer
    {
        let (num_sign, bytes) = self.to_bytes_le();
        let length = bytes.len();
        let sign: u8 = match num_sign {
            Sign::Plus => 0,
            Sign::Minus => 1,
            _ => panic!("Invalid bignum sign.")
        };
        let mut binary: Vec<u8> = match length {
            0...255 => {
                vec![BertTag::SmallBigNum as u8, length as u8, sign]
            },
            _ => {
                let mut binary = vec![BertTag::LargeBigNum as u8];
                binary.write_u32::<BigEndian>(length as u32).unwrap();
                binary.write_u8(sign).unwrap();
                binary
            }
        };
        binary.extend(bytes.iter().clone());

        let bytes: bytes::ByteBuf = binary.into();
        serializer.serialize_newtype_struct(BIGNUM_STRUCT_NAME, bytes)
    }
}