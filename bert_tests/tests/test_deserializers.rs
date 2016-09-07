extern crate bert;

use bert::{ETF_VERSION, binary_to_term};


#[test]
fn test_deserialize_u8() {
    let data = vec![ETF_VERSION, 97, 100];

    assert_eq!(
        100u8,
        binary_to_term(&data).unwrap()
    )
}


#[test]
fn test_deserialize_i32() {
    let data = vec![ETF_VERSION, 98, 0, 0, 2, 0];

    assert_eq!(
        512i32,
        binary_to_term(&data).unwrap()
    )
}


#[test]
fn test_deserialize_new_f64() {
    let data = vec![ETF_VERSION, 70, 64, 9, 30, 184, 81, 235, 133, 31];

    assert_eq!(
        3.14f64,
        binary_to_term(&data).unwrap()
    )
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
    )
}