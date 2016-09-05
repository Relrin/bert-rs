#![cfg_attr(test, feature(plugin, custom_derive))]
#![cfg_attr(test, plugin(serde_macros))]

extern crate byteorder;
extern crate num;
extern crate serde;

pub use deserializers::{
    Deserializer,
    binary_to_term, from_slice, from_reader
};
pub use errors::{Error, Result};
pub use serializers::{
    Serializer,
    term_to_binary, to_vec, to_writer
};
pub use types::{
    BERT_LABEL, ETF_VERSION,
    BertTag, BertBigInteger, BertTime, BertRegex,
    TimeStruct, RegexStruct, RegexOption,
};
pub use utils::{
    merge_terms, str_to_binary,
    get_atom, get_nil, get_bert_nil, get_bert_atom, get_empty_tuple,
    get_small_tuple
};

#[macro_use]
mod enum_macro;

#[macro_use]
mod forward;

mod deserializers;
mod errors;
mod serializers;
mod types;
mod wrappers;
mod utils;
