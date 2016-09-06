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
