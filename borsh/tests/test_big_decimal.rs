#![cfg(feature = "bigdecimal")]
use bigdecimal::BigDecimal;
use borsh::{BorshDeserialize, BorshSerialize};
use std::str::FromStr;

#[test]
fn test_bigdecimal() {
    let bigdecimals = vec![
        BigDecimal::from(0),
        BigDecimal::from_str("-0.0").unwrap(),
        BigDecimal::from_str("3.14159265358979323846").unwrap(),
        BigDecimal::from(256),
        BigDecimal::from(666),
        BigDecimal::from(-42),
        BigDecimal::from_str(&"7".repeat(1024)).unwrap(),
    ];
    for bigdecimal in bigdecimals {
        let serialized = bigdecimal.try_to_vec().unwrap();
        let deserialized =
            <BigDecimal>::try_from_slice(&serialized).expect("failed to deserialize BigDecimal");

        assert_eq!(deserialized, bigdecimal);
    }
}

#[test]
fn fuzz_failure() {
    let xs = vec![
        0, 4, 0, 0, 0, 255, 255, 255, 255, 15, 6, 32, 193, 193, 193, 255, 187,
    ];
    let ys = vec![
        0, 19, 0, 0, 0, 0, 0, 0, 0, 107, 255, 255, 191, 255, 13, 182, 1, 0, 0, 0, 1, 255, 255, 71,
        71, 71, 0, 35, 237, 255, 255, 118,
    ];
    let xs = BigDecimal::try_from_slice(&xs).unwrap();
    let ys = BigDecimal::try_from_slice(&ys).unwrap();
    let _ = xs == ys; // This overflows, please minimize and report a bug upstream, agains bigdecimal
}

#[test]
fn fuzz_failure_I_wanted_to_find_but_didnt() {
    use bigdecimal::num_bigint::BigInt;

    let xs = vec![1, 1, 0, 0, 0, 0];
    let xs = BigInt::try_from_slice(&xs).unwrap();
    let ys = vec![1, 0, 0, 0, 0];
    let ys = BigInt::try_from_slice(&ys).unwrap();
    assert!(xs != ys);
}
