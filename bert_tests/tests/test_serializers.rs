extern crate bert;
extern crate num;
extern crate serde;

use std::collections::{HashMap, BTreeMap};

use bert::{
    Serializer, term_to_binary,
    BertTag, BertBigInteger, BertTime
};
use num::bigint::{BigInt};
use serde::bytes::{Bytes};


#[test]
fn test_generate_term() {
    let mut writer = Vec::with_capacity(128);
    let mut bert = Serializer::new(&mut writer);

    let data: Vec<u8> = vec![0, 4, 116, 101, 115, 116];  // bert as string
    bert.generate_term(BertTag::Atom, data).unwrap();
    assert_eq!(
        *bert.into_inner(),
        vec![100u8, 0, 4, 116, 101, 115, 116]
    );
}


#[test]
fn test_merge_terms() {
    let mut writer = Vec::with_capacity(128);
    let bert = Serializer::new(&mut writer);

    let term_1: Vec<u8> = vec![100, 0, 4,  98, 101, 114, 116];
    let term_2: Vec<u8> = vec![100, 0, 3, 110, 105, 108];
    assert_eq!(
        bert.merge_terms(term_1, term_2),
        vec![
            100u8, 0, 4, 98, 101, 114, 116,  // "bert" as atom
            100,   0, 3, 110, 105, 108       // "nil" as atom
        ]
    );
}


#[test]
fn test_get_atom() {
    let mut writer = Vec::with_capacity(128);
    let bert = Serializer::new(&mut writer);

    assert_eq!(
        bert.get_atom("test"),
        vec![100u8, 0, 4, 116, 101, 115, 116]
    );
}


#[test]
fn test_get_nil() {
    let mut writer = Vec::with_capacity(128);
    let bert = Serializer::new(&mut writer);

    assert_eq!(
        bert.get_nil(),
        vec![106u8]
    );
}


#[test]
fn test_get_bert_nil() {
    let mut writer = Vec::with_capacity(128);
    let bert = Serializer::new(&mut writer);

    assert_eq!(
        bert.get_bert_nil(),
        vec![
            104u8,
            2,                              // tuple length
            100, 0, 4,  98, 101, 114, 116,  // "bert" as atom
            100, 0, 3, 110, 105, 108        // "nil" as atom
        ]
    );
}


#[test]
fn test_get_bert_atom() {
    let mut writer = Vec::with_capacity(128);
    let bert = Serializer::new(&mut writer);

    assert_eq!(
        bert.get_bert_atom(),
        vec![100, 0, 4,  98, 101, 114, 116] // "bert" as atom
    );
}


#[test]
fn test_serialize_bool() {
    assert_eq!(
        term_to_binary(&true).unwrap(),
        vec![
            131u8,
            104,                            // small tuple tag
            2,                              // tuple length
            100, 0, 4,  98, 101, 114, 116,  // "bert" as atom
            100, 0, 4, 116, 114, 117, 101   // "true" as atom
        ]
    );

    assert_eq!(
        term_to_binary(&false).unwrap(),
        vec![
            131u8,
            104,                               // small tuple tag
            2,                                 // tuple length
            100, 0, 4, 98, 101, 114, 116,      // "bert" as atom
            100, 0, 5, 102, 97, 108, 115, 101  // "false" as atom
        ]
    );
}


#[test]
fn test_serialize_isize() {
    let value: isize = 100;

    assert_eq!(
        term_to_binary(&value).unwrap(),
        vec![131u8, 98, 0, 0, 0, 100]
    )
}


#[test]
fn test_serialize_i8() {
    assert_eq!(
        term_to_binary(&-128i8).unwrap(),
        vec![131u8, 98, 255, 255, 255, 128]
    );

    assert_eq!(
        term_to_binary(&-1i8).unwrap(),
        vec![131u8, 98, 255, 255, 255, 255]
    );

    assert_eq!(
        term_to_binary(&64i8).unwrap(),
        vec![131u8, 98, 0, 0, 0, 64]
    );

    assert_eq!(
        term_to_binary(&127i8).unwrap(),
        vec![131u8, 98, 0, 0, 0, 127]
    );
}


#[test]
fn test_serialize_i16() {
    assert_eq!(
        term_to_binary(&-32768i16).unwrap(),
        vec![131u8, 98, 255, 255, 128, 0]
    );

    assert_eq!(
        term_to_binary(&-1i16).unwrap(),
        vec![131u8, 98, 255, 255, 255, 255]
    );

    assert_eq!(
        term_to_binary(&512i16).unwrap(),
        vec![131u8, 98, 0, 0, 2, 0]
    );

    assert_eq!(
        term_to_binary(&32767i16).unwrap(),
        vec![131u8, 98, 0, 0, 127, 255]
    );
}


#[test]
fn test_serialize_i32() {
    assert_eq!(
        term_to_binary(&-2147483648i32).unwrap(),
        vec![131u8, 98, 128, 0, 0, 0]
    );

    assert_eq!(
        term_to_binary(&-1i32).unwrap(),
        vec![131u8, 98, 255, 255, 255, 255]
    );

    assert_eq!(
        term_to_binary(&512i32).unwrap(),
        vec![131u8, 98, 0, 0, 2, 0]
    );

    assert_eq!(
        term_to_binary(&2147483647i32).unwrap(),
        vec![131u8, 98, 127, 255, 255, 255]
    );
}


#[test]
#[should_panic]
fn test_serialize_i64() {
    term_to_binary(&1000i64).unwrap();
}


#[test]
fn test_serialize_usize() {
    let value: usize = 100;

    assert_eq!(
        term_to_binary(&value).unwrap(),
        vec![
            131u8,
            98, 0, 0, 0, 100  // 100
        ]
    )
}


#[test]
fn test_serialize_u8() {
    assert_eq!(
        term_to_binary(&1u8).unwrap(),
        vec![131u8, 97, 1]
    );

    assert_eq!(
        term_to_binary(&255u8).unwrap(),
        vec![131u8, 97, 255]
    );
}


#[test]
#[should_panic]
fn test_serialize_u16() {
    term_to_binary(&100u16).unwrap();
}


#[test]
#[should_panic]
fn test_serialize_u32() {
    term_to_binary(&100u32).unwrap();
}


#[test]
#[should_panic]
fn test_serialize_u64() {
    term_to_binary(&100u64).unwrap();
}


#[test]
fn test_serialize_f32() {
    assert_eq!(
        term_to_binary(&-3.14f32).unwrap(),
        vec![131u8, 70, 192, 9, 30, 184, 96, 0, 0, 0]
    );

    assert_eq!(
        term_to_binary(&0.0f32).unwrap(),
        vec![131u8, 70, 0, 0, 0, 0, 0, 0, 0, 0]
    );

    assert_eq!(
        term_to_binary(&3.14f32).unwrap(),
        vec![131u8, 70, 64, 9, 30, 184, 96, 0, 0, 0]
    );
}


#[test]
fn test_serialize_f64() {
    assert_eq!(
        term_to_binary(&-3.14f64).unwrap(),
        vec![131u8, 70, 192, 9, 30, 184, 81, 235, 133, 31]
    );

    assert_eq!(
        term_to_binary(&0.0f64).unwrap(),
        vec![131u8, 70, 0, 0, 0, 0, 0, 0, 0, 0]
    );

    assert_eq!(
        term_to_binary(&3.14f64).unwrap(),
        vec![131u8, 70, 64, 9, 30, 184, 81, 235, 133, 31]
    );
}


#[test]
fn test_serialize_char() {
    assert_eq!(
        term_to_binary(&'a').unwrap(),
        vec![131u8, 107, 0, 1, 97]
    );
}


#[test]
fn test_serialize_string() {
    assert_eq!(
        term_to_binary(&"test").unwrap(),
        vec![131u8, 107, 0, 4, 116, 101, 115, 116]
    );
}

// TODO: Fix tests after adding specialization support in Rust
#[test]
fn test_serialize_bytes() {
    let empty_bytes_list: Bytes = b""[..].into();

    assert_eq!(
        term_to_binary(&empty_bytes_list).unwrap(),
        vec![
            131u8,
            109,         // binary
            0, 0, 0, 0   // length
        ]
    );

    let bytes_array: Bytes = b"value"[..].into();

    assert_eq!(
        term_to_binary(&bytes_array).unwrap(),
        vec![
            131u8,
            109,         // binary
            0, 0, 0, 5,  // length
            118,         // "v"
            97,          // "a"
            108,         // "l"
            117,         // "u"
            101          // "e"
        ]
    );
}


#[test]
fn test_serialize_tuple() {
    let small_tuple = (1u8, 4i32, 8.1516f64, String::from("value"));

    assert_eq!(
        term_to_binary(&small_tuple).unwrap(),
        vec![
            131u8,
            104,                                    // tuple
            4,                                      // length
            97, 1,                                  // 1
            98, 0, 0, 0, 4,                         // 4
            70, 64, 32, 77, 158, 131, 228, 37, 175, // 8.1516
            107, 0, 5, 118, 97, 108, 117, 101       // "value" as string
        ]
    );
}


#[test]
fn test_serialize_list() {
    let empty_list: Vec<i32> = vec![];

    assert_eq!(
        term_to_binary(&empty_list).unwrap(),
        vec![
            131u8,
            104,                            // tuple
            2,                              // tuple length
            100, 0, 4,  98, 101, 114, 116,  // "bert" as atom
            100, 0, 3, 110, 105, 108        // "nil" as atom
        ]
    );

    let list = [1i32, 2, 3];

    assert_eq!(
        term_to_binary(&list).unwrap(),
        vec![
            131u8,
            108,                            // list
            0, 0, 0, 3,                     // length
            98, 0, 0, 0, 1,                 // 1
            98, 0, 0, 0, 2,                 // 2
            98, 0, 0, 0, 3,                 // 3
            106                             // "nil" as atom
        ]
    );
}


#[test]
fn test_serialize_newtype_struct() {

    #[derive(Serialize)]
    struct Meters(i32);
    let distance = Meters(1000);

    assert_eq!(
        term_to_binary(&distance).unwrap(),
        vec![
            131,
            104,                                      // tuple
            2,                                        // length
            100, 0, 6, 109, 101, 116, 101, 114, 115,  // "meters" as atom
            98, 0, 0, 3, 232                          // 1000
        ]
    )
}


#[test]
fn test_serialize_newtype_variant() {

    #[derive(Serialize)]
    enum Enum {
        Inches(u8),
    }
    let variant = Enum::Inches(128);

    assert_eq!(
        term_to_binary(&variant).unwrap(),
        vec![
            131u8,
            104,                                    // tuple
            2,                                      // length
            100, 0, 4, 101, 110, 117, 109,          // "enum" as atom
            104,                                    // tuple
            2,                                      // length
            100, 0, 6, 105, 110, 99, 104, 101, 115, // "inches" as atom
            97, 128                                 // 128
        ]
    );
}

#[test]
fn test_serialize_tuple_struct() {

    #[derive(Serialize)]
    struct Point2D(i32, i32);
    let point = Point2D(1, 2);

    assert_eq!(
        term_to_binary(&point).unwrap(),
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


#[test]
fn test_serialize_tuple_variant() {

    #[derive(Serialize)]
    enum Enum {
        Point2D(i32, i32),
    }
    let variant = Enum::Point2D(1, 2);

    assert_eq!(
        term_to_binary(&variant).unwrap(),
        vec![
            131u8,
            105,                                          // tuple
            0, 0, 0, 2,                                   // length
            100, 0, 4, 101, 110, 117, 109,                // "enum" as atom
            105,                                          // tuple
            0, 0, 0, 3,                                   // length
            100, 0, 7, 112, 111, 105, 110, 116, 50, 100,  // "point2d" as atom
            98, 0, 0, 0, 1,                               // 1
            98, 0, 0, 0, 2                                // 2
        ]
    );
}


#[test]
fn test_serialize_struct() {

    #[derive(Serialize)]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }
    let color = Color{r: 128, g: 128, b: 128};

    assert_eq!(
        term_to_binary(&color).unwrap(),
        vec![
            131u8,
            105,                                // tuple
            0, 0, 0, 4,                         // length
            100, 0, 5, 99, 111, 108, 111, 114,  // "color" as atom

            104,                                // tuple
            2,                                  // length
            100, 0, 1, 114,                     // "r" as atom
            97, 128,                            // 128

            104,                                // tuple
            2,                                  // length
            100, 0, 1, 103,                     // "g" as atom
            97, 128,                            // 128

            104,                                // tuple
            2,                                  // length
            100, 0, 1, 98,                      // "b" as atom
            97, 128                             // 128
        ]
    );
}


#[test]
fn test_serialize_struct_variant() {

    #[derive(Serialize)]
    enum Enum{
        Color {r: u8, g: u8, b: u8},
    };
    let variant = Enum::Color{r: 128, g: 128, b: 128};

    assert_eq!(
        term_to_binary(&variant).unwrap(),
        vec![
            131u8,
            105,                                // tuple
            0, 0, 0, 2,                         // length
            100, 0, 4, 101, 110, 117, 109,      // "enum" as atom

            105,                                // tuple
            0, 0, 0, 4,                         // length
            100, 0, 5, 99, 111, 108, 111, 114,  // "color" as atom

            104,                                // tuple
            2,                                  // length
            100, 0, 1, 114,                     // "r" as atom
            97, 128,                            // 128

            104,                                // tuple
            2,                                  // length
            100, 0, 1, 103,                     // "g" as atom
            97, 128,                            // 128

            104,                                // tuple
            2,                                  // length
            100, 0, 1, 98,                      // "b" as atom
            97, 128                             // 128
        ]
    );
}



#[test]
fn test_serialize_map() {

    // empty map
    let empty_map: HashMap<&str, isize> = HashMap::new();

    assert_eq!(
        term_to_binary(&empty_map).unwrap(),
        vec![
            131u8,
            104,                            // tuple
            3,                              // length
            100, 0, 4, 98, 101, 114, 116,   // "bert" as atom
            100, 0, 4, 100, 105, 99, 116,   // "dict" as atom
            106                             // nil as []
        ]
    );

    // map with few pairs of elements
    let mut map: BTreeMap<&str, isize> = BTreeMap::new();
    map.insert("test", 4);
    map.insert("value", 5);

    assert_eq!(
        term_to_binary(&map).unwrap(),
        vec![
            131u8,
            104,                                // tuple
            3,                                  // length
            100, 0, 4, 98, 101, 114, 116,       // "bert" as atom
            100, 0, 4, 100, 105, 99, 116,       // "dict" as atom

            108,                                // list
            0, 0, 0, 2,                         // length

            104,                                // tuple
            2,                                  // length
            107, 0, 4, 116, 101, 115, 116,      // "value" as string
            98, 0, 0, 0, 4,                     // 4

            104,                                // tuple
            2,                                  // length
            107, 0, 5, 118, 97, 108, 117, 101,  // "value" as string
            98, 0, 0, 0, 5,                     // 5

            106                                 // nil
        ]
    );
}


#[test]
fn test_serialize_bignum() {
    let positive_bignum = BertBigInteger(BigInt::from(1000i32));

    assert_eq!(
        term_to_binary(&positive_bignum).unwrap(),
        vec![131u8, 110, 2, 0, 232, 3]
    );

    let negative_bignum = BertBigInteger(BigInt::from(-1000i32));

    assert_eq!(
        term_to_binary(&negative_bignum).unwrap(),
        vec![131u8, 110, 2, 1, 232, 3]
    );
}


#[test]
fn test_serialize_bert_time() {
    let time = BertTime::new(1255, 295581, 446228);

    assert_eq!(
        term_to_binary(&time).unwrap(),
        vec![
            131u8,
            104,                            // tuple
            5,                              // length
            100, 0, 4, 98, 101, 114, 116,   // "bert" as atom
            100, 0, 4, 116, 105, 109, 101,  // "time" as atom
            98, 0, 0, 4, 231,               // 1255
            98, 0, 4, 130, 157,             // 295581
            98, 0, 6, 207, 20               // 446228
        ]
    )
}
