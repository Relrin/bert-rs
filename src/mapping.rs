// Mapping for basic types, which will be interpreted into BERT types.
#[macro_use]
extern crate lazy_static;

use core::any::TypeId;
use std::collections::HashMap;

use types::BertType;


lazy_static! {
    static ref EncodeMapping: HashMap<TypeId, BertType> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
    };

    static ref DecodeMapping: HashMap<BertType, TypeId> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
    };
}

