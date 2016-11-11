# bert-rs
BERT (Binary ERlang Term) serializer

This crate provide an access to serializing data to the special binary data format, which can be send to your Erlang programs. The implementation relies on the [BERT](http://bert-rpc.org/) and [Erlang External Term Format](http://erlang.org/doc/apps/erts/erl_ext_dist.html) specifications.

Dependencies
------------
serde = "0.8.3"  
num = "0.1.34"  
byteorder = "0.5.3"  

License
-------
The bert-rs published under BSD license. For more details read [LICENSE](https://github.com/Relrin/bert-rs/blob/master/LICENSE) file.

Example of using
----------------
The bert-rs crate provide a support for default Rust types and some additional, which have specified in [BERT](http://bert-rpc.org/) document. For any supported type of data which should be serialized you will pass into `term_to_binary` function:

```rust
#![feature(proc_macro)]

extern crate bert;
extern crate serde;


#[derive(Debug, PartialEq, Serialize)]
struct Point2D(i32, i32);


fn main() {
    let point = Point2D(1, 2);

    // serialized to {point2d, 1, 2} in BERT format
    let serialized_point_2d = bert::term_to_binary(&point).unwrap(); 
    assert_eq!(
        serialized_point_2d
        vec![
            131u8,
            105,                                          // tuple
            0, 0, 0, 3,                                   // length
            100, 0, 7, 112, 111, 105, 110, 116, 50, 100,  // "point2d" as atom
            98, 0, 0, 0, 1,                               // 1
            98, 0, 0, 0, 2                                // 2
        ]   
    );
}
```

**Note**: At the moment bert-rs provide only serialize features. But bert-rs have the `serder-rs-deserializer` branch, where this library provide deserialize functionality. The part of required stuff is not implemented (because of issues with too complicated approaches of deserializing): list, tuples, `BertBigInteger` and special kind of tuples which represented as `{bert, ...}`. If you want to help in further development, then feel free to open pull requests and issues.


