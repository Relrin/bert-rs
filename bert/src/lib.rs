#![cfg_attr(test, feature(plugin, custom_derive))]
#![cfg_attr(test, plugin(serde_macros))]

extern crate byteorder;
extern crate num;
extern crate serde;

pub use errors::{Error, Result};
pub use serializers::{Serializer, term_to_binary, to_vec, to_writer};
pub use types::{
    BERT_LABEL, EXT_VERSION,
    BertTag, BertBigInteger, BertTime, BertRegex,
    TimeStruct, RegexStruct, RegexOption
};
pub use utils::{
    merge_terms, str_to_binary,
    get_atom, get_nil, get_bert_nil, get_bert_atom, get_empty_tuple,
    get_small_tuple
};


mod serializers;
mod types;
mod errors;
mod wrappers;
mod utils;