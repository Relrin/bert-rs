// For more information about basic "External Term Format" types you can read
// on the next page: http://erlang.org/doc/apps/erts/erl_ext_dist.html
extern crate anymap;

use std::string::String;
use std::vec::Vec;

use anymap::AnyMap;


pub const bert_label: &'static str = "bert";


// The BERT encoding is identical to Erlang's external term format except that
// it is restricted to the following data type identifiers: 97-100, 104-111.
#[derive(Debug, PartialEq)]
pub enum BertType {
    SmallInteger = 97, // 97, SMALL_INTEGER_EXT
    Integer = 98,      // 98, INTEGER_EXT
    Float = 99,        // 99, FLOAT_EXT
    Atom = 100,        // 100, ATOM_EXT

    SmallTuple = 104,  // 104, SMALL_TUPLE_EXT
    LargeTuple = 105,  // 105, LARGE_TUPLE_EXT
    Nil = 106,         // 106, NIL_EXT
    String = 107,      // 107, STRING_EXT
    List = 108,        // 108, LIST_EXT
    Binary = 109,      // 109, BINARY_EXT
    BigNum = 110,      // 110, SMALL_BIG_EXT
    LargeNum = 111,    // 111, LARGE_BIG_EXT
}


#[derive(Debug, PartialEq)]
pub struct BertNil {}


#[derive(Debug, PartialEq)]
pub struct BertBoolean {
    data: bool
}


#[derive(Debug, PartialEq)]
pub struct BertDictionary {
    data: AnyMap
}


#[derive(Debug, PartialEq)]
pub struct BertTime {
    megaseconds: i32,
    seconds: i32,
    microseconds: i32
}


#[derive(Debug, PartialEq)]
pub struct BertRegex {
    source: Vec<u8>,
    options: Vec<String>
}
