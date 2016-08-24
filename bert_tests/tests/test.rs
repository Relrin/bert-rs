#![cfg_attr(not(feature = "with-syntex"), feature(custom_derive, plugin))]
#![cfg_attr(not(feature = "with-syntex"), plugin(serde_macros))]

extern crate serde;
extern crate bert;

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/test.rs"));

#[cfg(not(feature = "with-syntex"))]
include!("test.rs.in");