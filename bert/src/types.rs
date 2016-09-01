// For more information about basic "External Term Format" types you can read
// on the next page: http://erlang.org/doc/apps/erts/erl_ext_dist.html
use std::string::String;
use std::ops::Deref;
use std::{i32};

use num::bigint::BigInt;

pub const BERT_LABEL: &'static str = "bert";
pub const EXT_VERSION: u8 = 131u8;


// The BERT encoding is identical to Erlang's external term format except that
// it is restricted to the following data type identifiers: 97-100, 104-111.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BertTag {
    NewFloat = 70,      // 70, NEW_FLOAT_EXT

    SmallInteger = 97,  // 97, SMALL_INTEGER_EXT
    Integer = 98,       // 98, INTEGER_EXT
    Float = 99,         // 99, FLOAT_EXT (deprecated; using for deserialize)
    Atom = 100,         // 100, ATOM_EXT

    SmallTuple = 104,   // 104, SMALL_TUPLE_EXT
    LargeTuple = 105,   // 105, LARGE_TUPLE_EXT
    Nil = 106,          // 106, NIL_EXT
    String = 107,       // 107, STRING_EXT
    List = 108,         // 108, LIST_EXT
    Binary = 109,       // 109, BINARY_EXT
    SmallBigNum = 110,  // 110, SMALL_BIG_EXT
    LargeBigNum = 111,  // 111, LARGE_BIG_EXT
}


#[derive(Debug, PartialEq)]
pub struct BertBigInteger(pub BigInt);


impl Deref for BertBigInteger {
    type Target = BigInt;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[derive(Debug, PartialEq)]
pub struct BertTime(TimeStruct);


impl BertTime {
    pub fn new(megaseconds: i32, seconds: i32, microseconds: i32) -> BertTime {
        BertTime(
            TimeStruct{
                megaseconds: megaseconds,
                seconds: seconds,
                microseconds: microseconds
            }
        )
    }
}


#[derive(Debug, PartialEq)]
pub struct TimeStruct {
    pub megaseconds: i32,
    pub seconds: i32,
    pub microseconds: i32,
}


impl Deref for BertTime {
    type Target = TimeStruct;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[derive(Debug, PartialEq)]
pub struct BertRegex(RegexStruct);


impl Deref for BertRegex {
    type Target = RegexStruct;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[derive(Debug, PartialEq)]
pub struct RegexStruct {
    pub source: String,
    pub options: Vec<RegexOption>
}


#[derive(Debug, PartialEq)]
pub enum RegexOption {
    Verbose,
    Ignorecase,
    Multiline,
    DotAll
}
