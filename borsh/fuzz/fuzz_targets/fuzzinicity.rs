#![no_main]
use bigdecimal::BigDecimal;
use libfuzzer_sys::fuzz_target;
use borsh::BorshDeserialize;

fuzz_target!(|data: (Vec<u8>, Vec<u8>)| {
    let (xs, ys) = data;
    if xs == ys {
        return;
    }
    let xs = BigDecimal::try_from_slice(&xs);
    let ys = BigDecimal::try_from_slice(&ys);
    match xs.ok().zip(ys.ok()) {
        Some((xs, ys)) => assert_ne!(xs, ys),
        _ => (),
    }
});
