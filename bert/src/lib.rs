#![cfg_attr(test, feature(plugin, custom_derive))]
#![cfg_attr(test, plugin(serde_macros))]

extern crate byteorder;
extern crate num;
extern crate serde;

pub use errors::{Error, Result};
pub use serializers::{Serializer, term_to_binary, to_vec, to_writer};
pub use types::{
    BERT_LABEL, EXT_VERSION,
    BertTag, BertTime, BertRegex
};

mod serializers;
mod types;
mod errors;
