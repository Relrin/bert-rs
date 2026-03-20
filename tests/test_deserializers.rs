use std::collections::BTreeMap;

use bert::{
    ETF_VERSION, binary_to_term, term_to_binary,
    BertBigInteger, BertTime, BertRegex, RegexOption,
};
use num::bigint::BigInt;
use serde::Deserialize;


// ─── Existing primitive deserialization tests ────────────────────────────────

#[test]
fn test_deserialize_u8() {
    let data = vec![ETF_VERSION, 97, 100];
    assert_eq!(100u8, binary_to_term(&data).unwrap());
}


#[test]
fn test_deserialize_i32() {
    let data = vec![ETF_VERSION, 98, 0, 0, 2, 0];
    assert_eq!(512i32, binary_to_term(&data).unwrap());
}


#[test]
fn test_deserialize_new_f64() {
    let data = vec![ETF_VERSION, 70, 64, 9, 30, 184, 81, 235, 133, 31];
    assert_eq!(3.14f64, binary_to_term(&data).unwrap());
}


#[test]
fn test_deserialize_old_f64() {
    let data = vec![
        ETF_VERSION, 99,
        53, 46, 53, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48,
        48, 48, 48, 101, 43, 48, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ];
    assert_eq!(5.5f64, binary_to_term(&data).unwrap());
}


#[test]
fn test_deserialize_atom() {
    let data = vec![ETF_VERSION, 100, 0, 3, 110, 105, 108];
    let result: String = binary_to_term(&data).unwrap();
    assert_eq!("nil", result);
}


#[test]
fn test_deserialize_string() {
    let data = vec![ETF_VERSION, 107, 0, 4, 98, 101, 114, 116];
    let result: String = binary_to_term(&data).unwrap();
    assert_eq!("bert", result);
}


#[test]
fn test_deserialize_binary() {
    let data = vec![
        ETF_VERSION,
        109, 0, 0, 0, 5,
        118, 97, 108, 117, 101
    ];
    let binary: Vec<u8> = binary_to_term(&data).unwrap();
    assert_eq!(b"value", binary.as_slice());
}


// ─── Bool deserialization ────────────────────────────────────────────────────

#[test]
fn test_deserialize_bool_true() {
    let data = vec![
        ETF_VERSION,
        104, 2,                              // SmallTuple, arity 2
        100, 0, 4,  98, 101, 114, 116,       // atom "bert"
        100, 0, 4, 116, 114, 117, 101        // atom "true"
    ];
    assert_eq!(true, binary_to_term(&data).unwrap());
}


#[test]
fn test_deserialize_bool_false() {
    let data = vec![
        ETF_VERSION,
        104, 2,                                // SmallTuple, arity 2
        100, 0, 4, 98, 101, 114, 116,          // atom "bert"
        100, 0, 5, 102, 97, 108, 115, 101      // atom "false"
    ];
    assert_eq!(false, binary_to_term(&data).unwrap());
}


// ─── List deserialization ────────────────────────────────────────────────────

#[test]
fn test_deserialize_list_i32() {
    let data = vec![
        ETF_VERSION,
        108,                            // List
        0, 0, 0, 3,                     // length 3
        98, 0, 0, 0, 1,                 // 1
        98, 0, 0, 0, 2,                 // 2
        98, 0, 0, 0, 3,                 // 3
        106                             // trailing Nil
    ];
    let result: Vec<i32> = binary_to_term(&data).unwrap();
    assert_eq!(vec![1, 2, 3], result);
}


#[test]
fn test_deserialize_list_string() {
    let data = vec![
        ETF_VERSION,
        108,                            // List
        0, 0, 0, 2,                     // length 2
        107, 0, 5, 104, 101, 108, 108, 111,  // "hello"
        107, 0, 5, 119, 111, 114, 108, 100,  // "world"
        106                             // trailing Nil
    ];
    let result: Vec<String> = binary_to_term(&data).unwrap();
    assert_eq!(vec!["hello".to_string(), "world".to_string()], result);
}


// ─── Tuple deserialization ───────────────────────────────────────────────────

#[test]
fn test_deserialize_small_tuple() {
    let data = vec![
        ETF_VERSION,
        104,                            // SmallTuple
        3,                              // arity 3
        97, 1,                          // u8: 1
        98, 0, 0, 0, 4,                // i32: 4
        70, 64, 32, 77, 158, 131, 228, 37, 175, // f64: 8.1516
    ];
    let result: (u8, i32, f64) = binary_to_term(&data).unwrap();
    assert_eq!((1u8, 4i32, 8.1516f64), result);
}


// ─── Map deserialization ─────────────────────────────────────────────────────

#[test]
fn test_deserialize_map_empty() {
    let data = vec![
        ETF_VERSION,
        104, 3,                              // SmallTuple, arity 3
        100, 0, 4, 98, 101, 114, 116,        // atom "bert"
        100, 0, 4, 100, 105, 99, 116,        // atom "dict"
        106                                  // Nil (empty list)
    ];
    let result: BTreeMap<String, i32> = binary_to_term(&data).unwrap();
    assert!(result.is_empty());
}


#[test]
fn test_deserialize_map() {
    let data = vec![
        ETF_VERSION,
        104, 3,                              // SmallTuple, arity 3
        100, 0, 4, 98, 101, 114, 116,        // atom "bert"
        100, 0, 4, 100, 105, 99, 116,        // atom "dict"

        108,                                 // List
        0, 0, 0, 2,                          // length 2

        104, 2,                              // SmallTuple, arity 2 (entry 1)
        107, 0, 4, 116, 101, 115, 116,       // "test"
        98, 0, 0, 0, 4,                      // 4

        104, 2,                              // SmallTuple, arity 2 (entry 2)
        107, 0, 5, 118, 97, 108, 117, 101,   // "value"
        98, 0, 0, 0, 5,                      // 5

        106                                  // trailing Nil
    ];
    let result: BTreeMap<String, i32> = binary_to_term(&data).unwrap();
    let mut expected = BTreeMap::new();
    expected.insert("test".to_string(), 4);
    expected.insert("value".to_string(), 5);
    assert_eq!(expected, result);
}


// ─── Struct deserialization ──────────────────────────────────────────────────

#[test]
fn test_deserialize_struct() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    let data = vec![
        ETF_VERSION,
        105,                                // LargeTuple
        0, 0, 0, 4,                         // arity 4
        100, 0, 5, 99, 111, 108, 111, 114,  // atom "color"

        104, 2,                              // SmallTuple(2)
        100, 0, 1, 114,                      // atom "r"
        97, 128,                             // 128

        104, 2,                              // SmallTuple(2)
        100, 0, 1, 103,                      // atom "g"
        97, 64,                              // 64

        104, 2,                              // SmallTuple(2)
        100, 0, 1, 98,                       // atom "b"
        97, 32,                              // 32
    ];
    let result: Color = binary_to_term(&data).unwrap();
    assert_eq!(Color { r: 128, g: 64, b: 32 }, result);
}


// ─── BigNum deserialization ──────────────────────────────────────────────────

#[test]
fn test_deserialize_small_bignum_positive() {
    let data = vec![
        ETF_VERSION,
        110,             // SmallBigNum
        2,               // length 2
        0,               // sign: positive
        232, 3           // 1000 in little-endian
    ];
    let result: BertBigInteger = binary_to_term(&data).unwrap();
    assert_eq!(BertBigInteger(BigInt::from(1000i32)), result);
}


#[test]
fn test_deserialize_small_bignum_negative() {
    let data = vec![
        ETF_VERSION,
        110,             // SmallBigNum
        2,               // length 2
        1,               // sign: negative
        232, 3           // 1000 in little-endian
    ];
    let result: BertBigInteger = binary_to_term(&data).unwrap();
    assert_eq!(BertBigInteger(BigInt::from(-1000i32)), result);
}


// ─── BertTime deserialization ────────────────────────────────────────────────

#[test]
fn test_deserialize_bert_time() {
    let data = vec![
        ETF_VERSION,
        104, 5,                              // SmallTuple, arity 5
        100, 0, 4, 98, 101, 114, 116,        // atom "bert"
        100, 0, 4, 116, 105, 109, 101,       // atom "time"
        98, 0, 0, 4, 231,                    // 1255
        98, 0, 4, 130, 157,                  // 295581
        98, 0, 6, 207, 20                    // 446228
    ];
    let result: BertTime = binary_to_term(&data).unwrap();
    assert_eq!(BertTime::new(1255, 295581, 446228), result);
}


// ─── BertRegex deserialization ───────────────────────────────────────────────

#[test]
fn test_deserialize_bert_regex() {
    let data = vec![
        ETF_VERSION,
        104, 4,                                          // SmallTuple, arity 4
        100, 0, 4, 98, 101, 114, 116,                    // atom "bert"
        100, 0, 5, 114, 101, 103, 101, 120,              // atom "regex"
        107, 0, 8, 94, 99, 40, 97, 42, 41, 116, 36,      // string "^c(a*)t$"

        108,                                              // List
        0, 0, 0, 1,                                       // length 1
        100, 0, 8, 99, 97, 115, 101, 108, 101, 115, 115,  // atom "caseless"
        106                                               // trailing Nil
    ];
    let result: BertRegex = binary_to_term(&data).unwrap();
    assert_eq!(BertRegex::new("^c(a*)t$", vec![RegexOption::Caseless]), result);
}


// ─── Option deserialization ──────────────────────────────────────────────────

#[test]
fn test_deserialize_option_none() {
    let data = vec![ETF_VERSION, 106]; // Nil
    let result: Option<i32> = binary_to_term(&data).unwrap();
    assert_eq!(None, result);
}


#[test]
fn test_deserialize_option_some() {
    let data = vec![ETF_VERSION, 98, 0, 0, 0, 42];
    let result: Option<i32> = binary_to_term(&data).unwrap();
    assert_eq!(Some(42), result);
}


// ─── Round-trip tests (serialize → deserialize) ──────────────────────────────

#[test]
fn test_roundtrip_bool() {
    let data = term_to_binary(&true).unwrap();
    assert_eq!(true, binary_to_term::<bool>(&data).unwrap());

    let data = term_to_binary(&false).unwrap();
    assert_eq!(false, binary_to_term::<bool>(&data).unwrap());
}


#[test]
fn test_roundtrip_u8() {
    let data = term_to_binary(&42u8).unwrap();
    assert_eq!(42u8, binary_to_term(&data).unwrap());
}


#[test]
fn test_roundtrip_i32() {
    let data = term_to_binary(&-12345i32).unwrap();
    assert_eq!(-12345i32, binary_to_term(&data).unwrap());
}


#[test]
fn test_roundtrip_f64() {
    let data = term_to_binary(&3.14159f64).unwrap();
    assert_eq!(3.14159f64, binary_to_term(&data).unwrap());
}


#[test]
fn test_roundtrip_string() {
    let data = term_to_binary(&"hello world").unwrap();
    let result: String = binary_to_term(&data).unwrap();
    assert_eq!("hello world", result);
}


#[test]
fn test_roundtrip_bytes() {
    let bytes = serde_bytes::ByteBuf::from(b"binary data".to_vec());
    let data = term_to_binary(&bytes).unwrap();
    let result: serde_bytes::ByteBuf = binary_to_term(&data).unwrap();
    assert_eq!(bytes, result);
}


#[test]
fn test_roundtrip_list() {
    let list: &[i32] = &[10, 20, 30];
    let data = term_to_binary(&list).unwrap();
    let result: Vec<i32> = binary_to_term(&data).unwrap();
    assert_eq!(vec![10, 20, 30], result);
}


#[test]
fn test_roundtrip_tuple() {
    let tuple = (1u8, 2i32, 3.0f64);
    let data = term_to_binary(&tuple).unwrap();
    let result: (u8, i32, f64) = binary_to_term(&data).unwrap();
    assert_eq!(tuple, result);
}


#[test]
fn test_roundtrip_map() {
    let mut map = BTreeMap::new();
    map.insert("alpha".to_string(), 1i32);
    map.insert("beta".to_string(), 2i32);
    let data = term_to_binary(&map).unwrap();
    let result: BTreeMap<String, i32> = binary_to_term(&data).unwrap();
    assert_eq!(map, result);
}


#[test]
fn test_roundtrip_struct() {
    #[derive(Debug, PartialEq, serde::Serialize, Deserialize)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: 20 };
    let data = term_to_binary(&point).unwrap();
    let result: Point = binary_to_term(&data).unwrap();
    assert_eq!(point, result);
}


#[test]
fn test_roundtrip_bignum() {
    let positive = BertBigInteger(BigInt::from(123456789i64));
    let data = term_to_binary(&positive).unwrap();
    let result: BertBigInteger = binary_to_term(&data).unwrap();
    assert_eq!(positive, result);

    let negative = BertBigInteger(BigInt::from(-987654321i64));
    let data = term_to_binary(&negative).unwrap();
    let result: BertBigInteger = binary_to_term(&data).unwrap();
    assert_eq!(negative, result);
}


#[test]
fn test_roundtrip_bert_time() {
    let time = BertTime::new(1255, 295581, 446228);
    let data = term_to_binary(&time).unwrap();
    let result: BertTime = binary_to_term(&data).unwrap();
    assert_eq!(time, result);
}


#[test]
fn test_roundtrip_bert_regex() {
    let regex = BertRegex::new("^c(a*)t$", vec![RegexOption::Caseless, RegexOption::Multiline]);
    let data = term_to_binary(&regex).unwrap();
    let result: BertRegex = binary_to_term(&data).unwrap();
    assert_eq!(regex, result);
}


#[test]
fn test_roundtrip_option_none() {
    let val: Option<i32> = None;
    let data = term_to_binary(&val).unwrap();
    let result: Option<i32> = binary_to_term(&data).unwrap();
    assert_eq!(None, result);
}


#[test]
fn test_roundtrip_option_some() {
    let val: Option<i32> = Some(42);
    let data = term_to_binary(&val).unwrap();
    let result: Option<i32> = binary_to_term(&data).unwrap();
    assert_eq!(Some(42), result);
}


// ─── Error case tests ───────────────────────────────────────────────────────

#[test]
fn test_error_invalid_version() {
    let data = vec![0, 97, 100]; // wrong version byte
    let result = binary_to_term::<u8>(&data);
    assert!(result.is_err());
}


#[test]
fn test_error_invalid_tag() {
    let data = vec![ETF_VERSION, 255]; // unknown tag
    let result = binary_to_term::<u8>(&data);
    assert!(result.is_err());
}


#[test]
fn test_error_trailing_bytes() {
    let data = vec![ETF_VERSION, 97, 100, 99]; // extra byte after u8 value
    let result = binary_to_term::<u8>(&data);
    assert!(result.is_err());
}
