extern crate bert;

use bert::{ETF_VERSION, binary_to_term};


#[test]
fn test_deserialize_u8() {
    let mut data = vec![ETF_VERSION, 97, 100];

    assert_eq!(
        binary_to_term(&data).unwrap(),
        100u8
    )
}
