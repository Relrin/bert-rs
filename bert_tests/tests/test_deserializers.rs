extern crate bert;

use std::string::{String};

use bert::{ETF_VERSION, binary_to_term};


#[test]
fn test_deserialize_u8() {
    let data = vec![ETF_VERSION, 97, 100];

    assert_eq!(
        100u8,
        binary_to_term(&data).unwrap()
    );
}


#[test]
fn test_deserialize_i32() {
    let data = vec![ETF_VERSION, 98, 0, 0, 2, 0];

    assert_eq!(
        512i32,
        binary_to_term(&data).unwrap()
    );
}


#[test]
fn test_deserialize_new_f64() {
    let data = vec![ETF_VERSION, 70, 64, 9, 30, 184, 81, 235, 133, 31];

    assert_eq!(
        3.14f64,
        binary_to_term(&data).unwrap()
    );
}


#[test]
fn test_deserialize_old_f64() {
    let data = vec![
        ETF_VERSION, 99,
        53, 46, 53, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48,
        48, 48, 48, 101, 43, 48, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ];

    assert_eq!(
        5.5f64,
        binary_to_term(&data).unwrap()
    );
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
        109,         // binary
        0, 0, 0, 5,  // length
        118,         // "v"
        97,          // "a"
        108,         // "l"
        117,         // "u"
        101          // "e"
    ];

    let binary: Vec<u8> = binary_to_term(&data).unwrap();
    assert_eq!(b"value", binary.as_slice());
}