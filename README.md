# bert-rs
BERT (Binary ERlang Term) serializer

This crate provide access to serializing data to the special binary data format, which can be send to your Erlang programs. The implementation relies on the [BERT](http://bert-rpc.org/) and [Erlang External Term Format](http://erlang.org/doc/apps/erts/erl_ext_dist.html) specifications.

Using
-----
Before you start working with this library you will need to add a link to the bert-rs library at your Cargo.toml file:
```toml
[dependencies]
bert = "0.2.0"
```

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

License
-------
The bert-rs published under BSD license. For more details read [LICENSE](https://github.com/Relrin/bert-rs/blob/master/LICENSE) file.
